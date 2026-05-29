import { PubSub } from "graphql-subscriptions";
import { PrismaClient, Attestation } from "@prisma/client";

export const pubsub = new PubSub();
export const ATTESTATION_CREATED = "ATTESTATION_CREATED";

type MappedAttestation = Omit<Attestation, "timestamp" | "expiration" | "createdAt" | "updatedAt"> & {
  timestamp: string;
  expiration: string | null;
  createdAt: string;
  updatedAt: string;
};

type PageInfo = {
  hasNextPage: boolean;
  hasPreviousPage: boolean;
  startCursor: string | null;
  endCursor: string | null;
};

type AttestationEdge = {
  node: MappedAttestation;
  cursor: string;
};

type AttestationConnection = {
  edges: AttestationEdge[];
  pageInfo: PageInfo;
  totalCount: number;
};

function mapAttestation(a: Attestation): MappedAttestation {
  return {
    ...a,
    timestamp: String(a.timestamp),
    expiration: a.expiration != null ? String(a.expiration) : null,
    createdAt: a.createdAt.toISOString(),
    updatedAt: a.updatedAt.toISOString(),
  };
}

function encodeCursor(id: string): string {
  return Buffer.from(id).toString('base64');
}

function decodeCursor(cursor: string): string {
  return Buffer.from(cursor, 'base64').toString('utf-8');
}

async function buildAttestationConnection(
  db: PrismaClient,
  where: Record<string, unknown>,
  first?: number,
  after?: string
): Promise<AttestationConnection> {
  const limit = Math.min(first || 50, 100); // Default 50, max 100
  
  // Get total count for the query
  const totalCount = await db.attestation.count({ where });
  
  // Build cursor-based query
  const cursorWhere = { ...where };
  if (after) {
    const decodedCursor = decodeCursor(after);
    cursorWhere.id = { gt: decodedCursor };
  }
  
  // Fetch one extra to determine hasNextPage
  const rows = await db.attestation.findMany({
    where: cursorWhere,
    orderBy: { id: "asc" },
    take: limit + 1,
  });
  
  const hasNextPage = rows.length > limit;
  const attestations = hasNextPage ? rows.slice(0, -1) : rows;
  
  const edges: AttestationEdge[] = attestations.map((attestation) => ({
    node: mapAttestation(attestation),
    cursor: encodeCursor(attestation.id),
  }));
  
  const pageInfo: PageInfo = {
    hasNextPage,
    hasPreviousPage: !!after,
    startCursor: edges.length > 0 ? edges[0].cursor : null,
    endCursor: edges.length > 0 ? edges[edges.length - 1].cursor : null,
  };
  
  return {
    edges,
    pageInfo,
    totalCount,
  };
}

export function buildResolvers(db: PrismaClient) {
  return {
    Query: {
      attestations: async (
        _: unknown,
        args: { 
          subject?: string; 
          claimType?: string; 
          status?: "ACTIVE" | "REVOKED";
          first?: number;
          after?: string;
        }
      ): Promise<AttestationConnection> => {
        const where: Record<string, unknown> = {};
        if (args.subject) where.subject = args.subject;
        if (args.claimType) where.claimType = args.claimType;
        if (args.status === "ACTIVE") where.isRevoked = false;
        if (args.status === "REVOKED") where.isRevoked = true;

        return buildAttestationConnection(db, where, args.first, args.after);
      },

      attestationsByIssuer: async (
        _: unknown,
        args: {
          issuer: string;
          first?: number;
          after?: string;
        }
      ): Promise<AttestationConnection> => {
        const where = { issuer: args.issuer };
        return buildAttestationConnection(db, where, args.first, args.after);
      },

      issuerStats: async (_: unknown, args: { issuer: string }) => {
        const rows = await db.attestation.findMany({
          where: { issuer: args.issuer },
          select: { isRevoked: true, claimType: true },
        });

        const claimTypes = [...new Set(rows.map((r) => r.claimType))];
        const revoked = rows.filter((r) => r.isRevoked).length;

        return {
          issuer: args.issuer,
          total: rows.length,
          active: rows.length - revoked,
          revoked,
          claimTypes,
        };
      },
    },

    Subscription: {
      onAttestationCreated: {
        subscribe: (_: unknown, args: { subject?: string }) => {
          const iter = pubsub.asyncIterableIterator<{
            onAttestationCreated: ReturnType<typeof mapAttestation>;
          }>(ATTESTATION_CREATED);

          if (!args.subject) return iter;

          // Filter by subject when provided
          const subject = args.subject;
          return {
            [Symbol.asyncIterator]() {
              return this;
            },
            async next(): Promise<IteratorResult<unknown>> {
              while (true) {
                const result = await iter.next();
                if (result.done) return result;
                const att = result.value?.onAttestationCreated;
                if (!att || att.subject === subject) return result;
              }
            },
            async return() {
              return iter.return?.() ?? { done: true as const, value: undefined };
            },
          };
        },
        resolve: (payload: {
          onAttestationCreated: ReturnType<typeof mapAttestation>;
        }) => payload.onAttestationCreated,
      },
    },
  };
}
