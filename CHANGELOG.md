# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.1.0 (2026-06-01)


### Features

* **#355:** add health check endpoints (/health and /ready) ([b63bbe9](https://github.com/marvs8/TrustLink/commit/b63bbe9562877890c94b76013c90ea5d6061fee3))
* **#356:** add Prometheus metrics endpoint (/metrics) ([ad5a757](https://github.com/marvs8/TrustLink/commit/ad5a757077119fbcacad7b7a120df0f99914650d))
* **#357:** add multi-sig proposal event indexing ([93ea49e](https://github.com/marvs8/TrustLink/commit/93ea49e77509a0aa14dd1f1402f643e04a5dd131))
* **#358:** add governance/DAO voting example contract ([158c644](https://github.com/marvs8/TrustLink/commit/158c644a6a63c35feb1f061499099b84b061bf7e))
* add ADR-008, ADR-009 and expand compliance.md for rate limiting, delegation, rectification, and data retention ([411c39a](https://github.com/marvs8/TrustLink/commit/411c39a1491036e28f68f199809ee0129c7de37a))
* Add attestation search by date range ([462cddd](https://github.com/marvs8/TrustLink/commit/462cdddb53e928a85727a0b4444a0b35a3235d48))
* Add attestation search by date range ([f4e0bc6](https://github.com/marvs8/TrustLink/commit/f4e0bc692c01ed9377e926c4ab35758f34a0237e))
* add attestation transfer for re-issuance by new issuer ([#25](https://github.com/marvs8/TrustLink/issues/25)) ([bc34684](https://github.com/marvs8/TrustLink/commit/bc34684101820623a9640942f473b80d40354d3e))
* add bridge contract trust model and Prometheus alerting rules ([3707a0a](https://github.com/marvs8/TrustLink/commit/3707a0a44fc529b5b37a52cf15cf549de67a7d9d))
* add claim type existence check with require_registered_claim_type config ([408fd28](https://github.com/marvs8/TrustLink/commit/408fd2861f475667c3d9a7266a239907a2c1f9fb))
* add claim_type length and character validation ([6ca81a8](https://github.com/marvs8/TrustLink/commit/6ca81a85a7d3b65f758aa0a329983203ccedc9ae))
* add claim_type length and character validation ([70ca658](https://github.com/marvs8/TrustLink/commit/70ca658a8fc6a533f4b0d6187403105f76d3e9a1))
* add CODEOWNERS, ValidAttestations index, and batch benchmarks ([7a4f0ef](https://github.com/marvs8/TrustLink/commit/7a4f0ef4e60ae7dde0a05f1df838a00c79b31eb2))
* add CODEOWNERS, ValidAttestations index, and batch benchmarks ([b177bc1](https://github.com/marvs8/TrustLink/commit/b177bc15c18a1efe7978e7a65ff2698a404ba80d)), closes [#593](https://github.com/marvs8/TrustLink/issues/593) [#594](https://github.com/marvs8/TrustLink/issues/594) [#596](https://github.com/marvs8/TrustLink/issues/596)
* add GDPR soft delete (request_deletion) for attestations ([cf7e5ee](https://github.com/marvs8/TrustLink/commit/cf7e5eeae593bcfce9550c15905a36007533283d))
* add GDPR soft delete (request_deletion) for attestations ([#299](https://github.com/marvs8/TrustLink/issues/299)) ([ae81be4](https://github.com/marvs8/TrustLink/commit/ae81be423a04ecdaea3455693905f86ea798db49))
* add get_bridge_list() paginated query ([b7d12af](https://github.com/marvs8/TrustLink/commit/b7d12afacee139d3b096f63ab372324c3deab7b4))
* add get_bridge_list() paginated query ([d51a4e6](https://github.com/marvs8/TrustLink/commit/d51a4e60ed356aa4acb34a643bf7129770d98ae8))
* add get_issuer_list() paginated query ([061a940](https://github.com/marvs8/TrustLink/commit/061a94043158054f271c679285783eebc2e45b09))
* add get_issuer_list() paginated query ([fa9d85b](https://github.com/marvs8/TrustLink/commit/fa9d85b1d913c23e445fceeefe2540c08e9eae08))
* add global stats integration test and fix pre-existing compilation errors ([4e768de](https://github.com/marvs8/TrustLink/commit/4e768de38fdfefb5b3701e727357acdc605e837b))
* add insurance policy underwriting example with KYC and AML verification ([9f1c885](https://github.com/marvs8/TrustLink/commit/9f1c885476626b29eacf200889c37ae4c2914a71))
* add insurance policy underwriting example with KYC and AML verification ([d788ca8](https://github.com/marvs8/TrustLink/commit/d788ca8d62bedb186c09bfbc55b6e70eb7c06202))
* add IssuerTier enforcement to attestation weight and logic ([de0f3f1](https://github.com/marvs8/TrustLink/commit/de0f3f1fafa507e5ae76046b2f7c6d90bb5089b9))
* add IssuerTier enforcement to attestation weight and logic ([#305](https://github.com/marvs8/TrustLink/issues/305)) ([3d7cf18](https://github.com/marvs8/TrustLink/commit/3d7cf186984e7662b2c050dd4bb23b435c381101))
* add jurisdiction fields to attestations ([02286ee](https://github.com/marvs8/TrustLink/commit/02286ee973b6fda3a6e91703381e92a73c940a8d))
* add jurisdiction fields to attestations ([0d66a5b](https://github.com/marvs8/TrustLink/commit/0d66a5b77175fc0ee67250dc48778397a11219ea))
* add pagination to get_attestations_by_jurisdiction ([c921454](https://github.com/marvs8/TrustLink/commit/c92145486188351c507fe023240623fb2df781cc))
* add pagination to get_attestations_by_jurisdiction ([#307](https://github.com/marvs8/TrustLink/issues/307)) ([1c8358c](https://github.com/marvs8/TrustLink/commit/1c8358c75e5c1922e2a4c22b8e085c296504b798))
* add pull-based attestation request workflow ([822e528](https://github.com/marvs8/TrustLink/commit/822e5289176decf60eec5404c11ecf7168031e7a))
* Add Python bindings with get_audit_log() and GraphQL pagination ([84e5b4d](https://github.com/marvs8/TrustLink/commit/84e5b4d2156b9d378c7bb251e5d65c516c88ce00))
* Add Python bindings with get_audit_log() and GraphQL pagination ([c5d0102](https://github.com/marvs8/TrustLink/commit/c5d0102d4aa1ad7e4f9175f23f5aac6e4cbd3c5e))
* add two-step admin transfer with pending confirmation ([cbb8ca1](https://github.com/marvs8/TrustLink/commit/cbb8ca16a068367e55a3120d829ad48c1b525e9e))
* add two-step admin transfer with pending confirmation ([#284](https://github.com/marvs8/TrustLink/issues/284)) ([b14d591](https://github.com/marvs8/TrustLink/commit/b14d591b442d7646b6d022d12088e8ad1529ea2b))
* add underflow-safe counter tracking for issuers, attestations, revocations ([81bdc4e](https://github.com/marvs8/TrustLink/commit/81bdc4ee5520f0ec880ebcdc60544b3cfe9ffb88))
* add validate_claim_type and comprehensive validation test suite ([2ea1b40](https://github.com/marvs8/TrustLink/commit/2ea1b4025c067ac6ba6265074be253ef9740682c))
* add validate_claim_type and comprehensive validation test suite ([90624b4](https://github.com/marvs8/TrustLink/commit/90624b4d74f0439874477a54ef98b10828dca50e))
* add wallet disconnect button ([001d19e](https://github.com/marvs8/TrustLink/commit/001d19e2882467f8be80318317d3d5849e4b4337))
* add wallet disconnect button ([3b4efe8](https://github.com/marvs8/TrustLink/commit/3b4efe8742af8b10d5bf4f35ed9e1f96e194b842))
* add whitelist mode for subject attestation control ([d74bfbe](https://github.com/marvs8/TrustLink/commit/d74bfbe5cc1f8a9c1e3397141ddba34417b0fb74))
* add whitelist mode for subject attestation control ([a2a9356](https://github.com/marvs8/TrustLink/commit/a2a9356921863dafb72e41101fe303c3b64d92b0))
* **attestation:** implement get_confidence_score(attestation_id) -&gt; u32 ([c7d51ff](https://github.com/marvs8/TrustLink/commit/c7d51fff2a47c6913655380a75c65b4637451325))
* **attestation:** implement get_confidence_score(attestation_id) -&gt; u32 ([d05cacc](https://github.com/marvs8/TrustLink/commit/d05cacc41159e800aadd138eabfde5082202f3c8)), closes [#297](https://github.com/marvs8/TrustLink/issues/297)
* **attestation:** implement transfer_attestation for compromised ([2bef451](https://github.com/marvs8/TrustLink/commit/2bef451a41a6cbf32c00aa5cedf67176e38d9f7d))
* **attestation:** implement transfer_attestation for compromised issuer recovery ([812964c](https://github.com/marvs8/TrustLink/commit/812964cee0592960643e8d48c79496b4fbaabfea))
* **bindings,indexer:** add input validation and monitoring improvements ([e88bd60](https://github.com/marvs8/TrustLink/commit/e88bd6012ee379ba5003eb16616c3e6ca009e42a))
* **bindings,indexer:** add input validation and monitoring improvements ([0f2da49](https://github.com/marvs8/TrustLink/commit/0f2da49250e67fd606f98c0fb2f85cdb50b013c5))
* **bindings:** add Python bindings for TrustLink contract ([#363](https://github.com/marvs8/TrustLink/issues/363)) ([cbe8063](https://github.com/marvs8/TrustLink/commit/cbe8063ca8754d054f0696b6beff13e48e851b68))
* **bindings:** document auto-generation of TypeScript bindings from contract ABI ([#362](https://github.com/marvs8/TrustLink/issues/362)) ([394a42c](https://github.com/marvs8/TrustLink/commit/394a42cf8cccfe137d350436e6aae03da429f88f))
* **endorsements:** implement get_endorsements and get_endorsement_count ([94483c1](https://github.com/marvs8/TrustLink/commit/94483c1470eac0c1ac41eb209481cadc30297b9b)), closes [#300](https://github.com/marvs8/TrustLink/issues/300)
* **endorsements:** implement get_endorsements() and get_endorsement_count() ([66d2944](https://github.com/marvs8/TrustLink/commit/66d2944c71618abb63d1315894d30c93f4956f9f))
* **examples:** add CLI tool for issuer operations ([#361](https://github.com/marvs8/TrustLink/issues/361)) ([3257325](https://github.com/marvs8/TrustLink/commit/3257325126f72dcae675535defbf6d3d1628bad8))
* **examples:** add Python server-side verification example ([#360](https://github.com/marvs8/TrustLink/issues/360)) ([34e9455](https://github.com/marvs8/TrustLink/commit/34e945542eda314bd3c78786b7d980a818e5f520))
* **examples:** complete anchor-integration flow with expiration handling ([#359](https://github.com/marvs8/TrustLink/issues/359)) ([813d75c](https://github.com/marvs8/TrustLink/commit/813d75c6b2b9374fe5af9abf7ba17b6f5b5114bc))
* expose expiration hook registration and notification flow ([a91e84c](https://github.com/marvs8/TrustLink/commit/a91e84cbb465251997e08991e629be1fa8da2d28))
* expose expiration hook registration and notification flow ([#319](https://github.com/marvs8/TrustLink/issues/319)) ([2c4f642](https://github.com/marvs8/TrustLink/commit/2c4f642895f15b68573ae986014902705eb9aaf4))
* expose get_pending_admin_transfer() as read-only query ([9018d0b](https://github.com/marvs8/TrustLink/commit/9018d0b44bf8d00129c7381c736fd780af248d78))
* Feat/expose get pending admin transfer ([c7a6cf2](https://github.com/marvs8/TrustLink/commit/c7a6cf25a12c769adc577cd1606855af3d727d0a))
* **governance:** implement M-of-N council quorum for sensitive admin operations ([94dcfad](https://github.com/marvs8/TrustLink/commit/94dcfad3f1571c8241681aab4d4ac247330d147a)), closes [#268](https://github.com/marvs8/TrustLink/issues/268)
* **governance:** implement M-of-N council quorum for sensitive admin… ([9a42821](https://github.com/marvs8/TrustLink/commit/9a4282120b6da8177ae605d63b8a986df498d1da))
* implement Add tests for admin council operations ([e94e739](https://github.com/marvs8/TrustLink/commit/e94e739fd5b42691b6d8ca879b02bd91e70de9a3))
* implement Add tests for claim type registry pagination ([4259fb7](https://github.com/marvs8/TrustLink/commit/4259fb7cf3d0181dd2e0ca6faef49a53fabb02ff))
* implement attestation templates (create, instantiate, list, get) ([149b872](https://github.com/marvs8/TrustLink/commit/149b87239fbe7253378f4243d0ed1bc8069dcae2))
* implement attestation templates (create, instantiate, list, get) ([dccfd75](https://github.com/marvs8/TrustLink/commit/dccfd75d5cd8b45b8a470277ca7ca11cb603abc3))
* implement attestation valid_from lifecycle with Pending status ([066f14f](https://github.com/marvs8/TrustLink/commit/066f14fdc253f19de078fa14793506cb23030c50))
* implement attestation valid_from lifecycle with Pending status ([7616c0c](https://github.com/marvs8/TrustLink/commit/7616c0c213257f3572e932cdf9b77b4acf8a844b))
* implement issuer delegation (sub-issuer authority) ([#298](https://github.com/marvs8/TrustLink/issues/298)) ([#431](https://github.com/marvs8/TrustLink/issues/431)) ([88259db](https://github.com/marvs8/TrustLink/commit/88259db01201f8f32232044126fa1d54f511a100))
* implement issuer whitelist mode ([#302](https://github.com/marvs8/TrustLink/issues/302)) ([fb398c9](https://github.com/marvs8/TrustLink/commit/fb398c9861ebf3396edf9ce85c9321d277ca9d26))
* implement issuer whitelist mode per issuer ([8e0d85d](https://github.com/marvs8/TrustLink/commit/8e0d85de1bd5f11cb26dc2c711d5b88c3f9c31e6))
* implement mainnet-checklist.md: add post-deployment verificatio… ([88d1e11](https://github.com/marvs8/TrustLink/commit/88d1e113094aeb40395ff7afb723380211a977dd))
* implement mainnet-checklist.md: add post-deployment verification steps ([9427268](https://github.com/marvs8/TrustLink/commit/94272689cb30286bf0456a01650fe9d435061eb6))
* implement revoke_attestations_batch with max 50 limit and reason ([398aaac](https://github.com/marvs8/TrustLink/commit/398aaac98d0f2b01fd8a805349950489eac8b39c))
* implement revoke_attestations_batch with max 50 limit and reason ([#295](https://github.com/marvs8/TrustLink/issues/295)) ([4faf53c](https://github.com/marvs8/TrustLink/commit/4faf53cef29e1f78b6be4311fcac9b54e5ad6611))
* **indexer:** add attestation request persistence and GraphQL query support ([66a811f](https://github.com/marvs8/TrustLink/commit/66a811f0d77b7b4de6e6d554bec21640c6ca5e8c)), closes [#545](https://github.com/marvs8/TrustLink/issues/545)
* **indexer:** add database indexes for common query patterns ([#352](https://github.com/marvs8/TrustLink/issues/352)) ([ec804ff](https://github.com/marvs8/TrustLink/commit/ec804ff5aefceecf9fe811f14e7db68216e952bb))
* **indexer:** add event replay from genesis for full historical sync ([#354](https://github.com/marvs8/TrustLink/issues/354)) ([8778ff6](https://github.com/marvs8/TrustLink/commit/8778ff67a22fff99b7ba1d39b47aede533c65c61))
* **indexer:** add GraphQL subscriptions for real-time events ([#351](https://github.com/marvs8/TrustLink/issues/351)) ([fd6c3ab](https://github.com/marvs8/TrustLink/commit/fd6c3ab4b93e80020ff4e1e0141396b7989da8e2))
* **indexer:** add issuer management, health checks, and reindex capabilities ([e1000d3](https://github.com/marvs8/TrustLink/commit/e1000d3dece19bd294ac90b7271c75bfc2deea8c))
* **indexer:** add issuer management, health checks, and reindex capabilities ([68fe00b](https://github.com/marvs8/TrustLink/commit/68fe00b9ca6a31f7f4ad7d7bb78ad72f9ba2cc2f))
* **indexer:** add multi-sig proposal persistence and GraphQL query support ([3c3a1c8](https://github.com/marvs8/TrustLink/commit/3c3a1c8a05c05013b34c826b839c66b4629f8e2d))
* **indexer:** add REST API endpoints alongside GraphQL ([#353](https://github.com/marvs8/TrustLink/issues/353)) ([c8fe448](https://github.com/marvs8/TrustLink/commit/c8fe4487030dfb3ff1cf8e9d33a938b1e4a462f3))
* **indexer:** durable webhook failure handling and recovery ([d7f5308](https://github.com/marvs8/TrustLink/commit/d7f53081b1fc808d38641062953b05713e217656)), closes [#545](https://github.com/marvs8/TrustLink/issues/545)
* make multisig proposal TTL configurable by admin ([5d57634](https://github.com/marvs8/TrustLink/commit/5d57634db518d275d79428105de31d3c28162067))
* make multisig proposal TTL configurable by admin ([#308](https://github.com/marvs8/TrustLink/issues/308)) ([ce0ac96](https://github.com/marvs8/TrustLink/commit/ce0ac96e3af57879a145a0471df28a2b6cae0314))
* **makefile:** add snapshot-update target and snapshot testing docs ([56bc97f](https://github.com/marvs8/TrustLink/commit/56bc97f3145cb8790f0bb80526cd56fd93514b09))
* **makefile:** add snapshot-update target and snapshot testing docs ([976270c](https://github.com/marvs8/TrustLink/commit/976270ca4dbd6527d25b48c3b180672b25119407))
* perf: add wasm-opt -Oz to build pipeline and document size reduction ([6960f12](https://github.com/marvs8/TrustLink/commit/6960f1297d3b0a8353a6eaddc3cc3b7cd4692521))
* Perf/chunked index storage ([076f21b](https://github.com/marvs8/TrustLink/commit/076f21bd9476274c1806adb7072b6c54eed7de8f))
* Perf/optimize batch attestation ([ba51d6a](https://github.com/marvs8/TrustLink/commit/ba51d6acc46da961f2cf15e6083cb9b937f3d5ab))
* Perf/storage cost benchmarks ([3277c7e](https://github.com/marvs8/TrustLink/commit/3277c7e37ac0f3b80aa1058d2bc3c865db02d181))
* **python-sdk:** add has_all_claims and has_any_claim with validation and tests ([b955812](https://github.com/marvs8/TrustLink/commit/b95581231ba3b4d3579bdf65715d06a3bdef19f5)), closes [#545](https://github.com/marvs8/TrustLink/issues/545)
* **python:** add AsyncTrustLinkClient for asyncio support ([71b0416](https://github.com/marvs8/TrustLink/commit/71b0416fd395e9b85420bc0a60147c4b8ecda687))
* **python:** add AsyncTrustLinkClient for asyncio support ([8b21631](https://github.com/marvs8/TrustLink/commit/8b2163152dd46b69d4c608ad75a13526017ea920)), closes [#540](https://github.com/marvs8/TrustLink/issues/540)
* **python:** prepare trustlink-sdk for PyPI distribution ([b4e576c](https://github.com/marvs8/TrustLink/commit/b4e576cbb6ba5dd6383981eb717219416d41b384))
* **python:** prepare trustlink-sdk for PyPI distribution ([7daa0fa](https://github.com/marvs8/TrustLink/commit/7daa0fae2635bbed8fe8359cdfd83fded1386eb6))
* **query:** add get_attestation_by_type() returning Option&lt;Attestation&gt; ([ba2033e](https://github.com/marvs8/TrustLink/commit/ba2033ef99b8bfa817e0fdfc3116441191016595))
* **query:** add get_attestation_by_type() returning Option&lt;Attestation&gt; ([c5aa1e6](https://github.com/marvs8/TrustLink/commit/c5aa1e602c105ab98a6387a9f3c7461fb7d93777)), closes [#296](https://github.com/marvs8/TrustLink/issues/296)
* **query:** add get_issuer_attestation_count() ([8e6a72c](https://github.com/marvs8/TrustLink/commit/8e6a72cda07d1fc7d4d55ae6769ad44ec672e292))
* **query:** add get_issuer_attestation_count() ([4888c89](https://github.com/marvs8/TrustLink/commit/4888c89094bf6d0a0b037e1a2e8883317478f76e)), closes [#306](https://github.com/marvs8/TrustLink/issues/306)
* **query:** add get_valid_claim_count() for a subject ([28b4053](https://github.com/marvs8/TrustLink/commit/28b4053da2640aa07204c4dfaf4fd9b8672583ba))
* **query:** add get_valid_claim_count() for a subject ([b8e6c90](https://github.com/marvs8/TrustLink/commit/b8e6c90bea7abdb1bdb13eab3c2ed14b88ebb213)), closes [#303](https://github.com/marvs8/TrustLink/issues/303)
* React dApp with 4 panels, Freighter wallet, GitHub Pages deploy ([a151e1b](https://github.com/marvs8/TrustLink/commit/a151e1bdb724f43f78a21c848ee4187c9327a3d7))
* **react-app:** add expiring attestations section with renewal to Is… ([528ba68](https://github.com/marvs8/TrustLink/commit/528ba684f48caae94f731d7de82ae52e23aa3fba))
* **react-app:** add expiring attestations section with renewal to IssuerDashboard ([043eda5](https://github.com/marvs8/TrustLink/commit/043eda5e221c581941193efb8cfc5dc244fa79af)), closes [#562](https://github.com/marvs8/TrustLink/issues/562)
* **react-app:** add useGlobalStats hook and refactor AdminPanel ([342e65d](https://github.com/marvs8/TrustLink/commit/342e65dbb0a921eb4f689de8d4886ac98839f374))
* **react-app:** add useGlobalStats hook and refactor AdminPanel ([dc4ba44](https://github.com/marvs8/TrustLink/commit/dc4ba44bfdee263cf5ad9651705f05d509cbf021)), closes [#539](https://github.com/marvs8/TrustLink/issues/539)
* **react:** add attestation request flow UI ([#364](https://github.com/marvs8/TrustLink/issues/364)) ([d553342](https://github.com/marvs8/TrustLink/commit/d5533427fd97a0cd1ee34d0a61431e9a6548c447))
* **react:** add issuer dashboard with stats ([#366](https://github.com/marvs8/TrustLink/issues/366)) ([fb94038](https://github.com/marvs8/TrustLink/commit/fb940389935170feff7e9832d96326d993403d78))
* **react:** add multi-sig proposal UI ([#365](https://github.com/marvs8/TrustLink/issues/365)) ([2b4917e](https://github.com/marvs8/TrustLink/commit/2b4917ee8924c1907406c38d0df2dfc982c2f256))
* replace attestation booleans with origin enum ([05726dd](https://github.com/marvs8/TrustLink/commit/05726dda005ea0e14940dba06015fbefdd675fd8))
* replace attestation booleans with origin enum ([f1d8f2c](https://github.com/marvs8/TrustLink/commit/f1d8f2c5ae661fc23856cbb521301ff597024178))
* **requests:** implement attestation request workflow ([5618e89](https://github.com/marvs8/TrustLink/commit/5618e8965789f2848a8e01a22541051c09271fae))
* **requests:** implement attestation request workflow ([#304](https://github.com/marvs8/TrustLink/issues/304)) ([465f535](https://github.com/marvs8/TrustLink/commit/465f535f776b8af092bf7ac1265f29c1c104a824))
* resolve issues [#506](https://github.com/marvs8/TrustLink/issues/506) [#507](https://github.com/marvs8/TrustLink/issues/507) [#508](https://github.com/marvs8/TrustLink/issues/508) [#509](https://github.com/marvs8/TrustLink/issues/509) ([5853e26](https://github.com/marvs8/TrustLink/commit/5853e26416fa73590e8d7fb0a56c5d0b7c85c687))
* resolve issues [#506](https://github.com/marvs8/TrustLink/issues/506) [#507](https://github.com/marvs8/TrustLink/issues/507) [#508](https://github.com/marvs8/TrustLink/issues/508) [#509](https://github.com/marvs8/TrustLink/issues/509) ([c69deac](https://github.com/marvs8/TrustLink/commit/c69deacaae1415000ec890ff4dc35e0575d796aa))
* resolve issues [#526](https://github.com/marvs8/TrustLink/issues/526), [#527](https://github.com/marvs8/TrustLink/issues/527), [#528](https://github.com/marvs8/TrustLink/issues/528), [#529](https://github.com/marvs8/TrustLink/issues/529) ([d003daa](https://github.com/marvs8/TrustLink/commit/d003daaca983455a17f412bba7c99449e42462f7))
* resolve issues [#526](https://github.com/marvs8/TrustLink/issues/526), [#527](https://github.com/marvs8/TrustLink/issues/527), [#528](https://github.com/marvs8/TrustLink/issues/528), [#529](https://github.com/marvs8/TrustLink/issues/529) ([b7f2319](https://github.com/marvs8/TrustLink/commit/b7f23198b9ca9bf7e81f46531b55e157ab475a7a))
* resolve issues [#530](https://github.com/marvs8/TrustLink/issues/530), [#531](https://github.com/marvs8/TrustLink/issues/531), [#532](https://github.com/marvs8/TrustLink/issues/532) — template deletion, tier claims SDK fix, attestation analytics ([7a35714](https://github.com/marvs8/TrustLink/commit/7a35714b7b0b37545151c7135183b56bd08658ef))
* resolve issues [#530](https://github.com/marvs8/TrustLink/issues/530), [#531](https://github.com/marvs8/TrustLink/issues/531), [#532](https://github.com/marvs8/TrustLink/issues/532) — templates, tier claims, analytics ([1efb88b](https://github.com/marvs8/TrustLink/commit/1efb88b7ad71669a7e8816ecde249ffe1c4b1a87))
* **sdk/react:** add useIssuerStats hook and refactor IssuerDashboard ([e8e5fb3](https://github.com/marvs8/TrustLink/commit/e8e5fb35ae7b78f499bb8e13edd1a0a73909e97d))
* **sdk/react:** add useIssuerStats hook and refactor IssuerDashboard ([502260b](https://github.com/marvs8/TrustLink/commit/502260ba5db0bdf7361a472d57b86aec71568421)), closes [#538](https://github.com/marvs8/TrustLink/issues/538)
* **sdk:** add missing contract methods to TypeScript client ([a782355](https://github.com/marvs8/TrustLink/commit/a78235542de7d5fcc6f82c92adf00c6a3b459e00))
* **sdk:** add missing contract methods to TypeScript client ([16e2402](https://github.com/marvs8/TrustLink/commit/16e2402b4ec5f78de82fa648bf577597b302837c))
* **sdk:** add React hooks package ([6a2533e](https://github.com/marvs8/TrustLink/commit/6a2533e86453009a37c04b0e35dfb1669c39ab7c))
* **sdk:** add React hooks package ([48eb64d](https://github.com/marvs8/TrustLink/commit/48eb64da68dcabed2fc536e6322aeeffd72be49c)), closes [#350](https://github.com/marvs8/TrustLink/issues/350)
* **sdk:** add ResilienceConfig, provenance, iterateSubjectAttestations docs, and TypeDoc generation ([6d3e152](https://github.com/marvs8/TrustLink/commit/6d3e1526299e6d9aba40b192cb2bc73e83b39a93))
* **sdk:** add typed error classes to TypeScript client ([fe3c320](https://github.com/marvs8/TrustLink/commit/fe3c320e749cf8505737b2adecd9158d320d5087))
* **sdk:** add typed error classes to TypeScript client ([22d9442](https://github.com/marvs8/TrustLink/commit/22d94423ad45ee79ee3eb580fd036ec2894623b2)), closes [#347](https://github.com/marvs8/TrustLink/issues/347)
* **sdk:** ResilienceConfig, pagination docs, npm provenance, TypeDoc — closes [#534](https://github.com/marvs8/TrustLink/issues/534) [#535](https://github.com/marvs8/TrustLink/issues/535) [#536](https://github.com/marvs8/TrustLink/issues/536) [#537](https://github.com/marvs8/TrustLink/issues/537) ([496fa5a](https://github.com/marvs8/TrustLink/commit/496fa5a053600a64515ab181568cb64dfbdbb3c1))
* **sdk:** sync TypeScript types with Rust contract types ([59fc364](https://github.com/marvs8/TrustLink/commit/59fc3642aef155a7e433a344a4cad6630c56dc16))
* **sdk:** sync TypeScript types with Rust contract types ([1b5fbe4](https://github.com/marvs8/TrustLink/commit/1b5fbe4ff15cd054fe8e3ff11c288dbd94fe0949))
* **security:** add cargo-deny integration and dependency security policy ([5ee39fb](https://github.com/marvs8/TrustLink/commit/5ee39fb7a9b3291c64859d78295e29d25223cdfb))
* **security:** add cargo-deny integration and dependency security policy ([8262d07](https://github.com/marvs8/TrustLink/commit/8262d07c81d0c6ca8f36037c40cc3f8c89d6b59a))
* **tiers:** add IssuerTier enforcement to attestation weight ([b54ce9c](https://github.com/marvs8/TrustLink/commit/b54ce9c85b98a1a708622fe205ea6b8baebbe2bf))
* **tiers:** add IssuerTier enforcement to attestation weight ([#305](https://github.com/marvs8/TrustLink/issues/305)) ([1674ddc](https://github.com/marvs8/TrustLink/commit/1674ddc3c6a61b66a40f23637927cf3a1bb5b039))
* underflow-safe counters for issuers, attestations, and revocations ([d290b5d](https://github.com/marvs8/TrustLink/commit/d290b5d0799c728e4c62e8c0897245f155848858))
* validate jurisdiction field against ISO 3166-1 alpha-2 codes ([a373287](https://github.com/marvs8/TrustLink/commit/a373287c28c577f575bfc625aaf420693b8be0bd))


### Bug Fixes

* **#533:** renew_attestation records new expiration in audit log details ([f7a1440](https://github.com/marvs8/TrustLink/commit/f7a1440032f1c6ac6e6e7e5f0c70af515e08057a))
* **#533:** renew_attestation records new expiration in audit log details ([a749f51](https://github.com/marvs8/TrustLink/commit/a749f515cd5086d1c50f1f3b400a17035b56b746))
* **#558, #559:** add dark mode toggle and getAttestationsByTag pagina… ([cfad08b](https://github.com/marvs8/TrustLink/commit/cfad08bc44fcc106abc3b3efc650a114761abe19))
* **#558, #559:** add dark mode toggle and getAttestationsByTag pagination ([8c033b2](https://github.com/marvs8/TrustLink/commit/8c033b25d5c6d84678990d450e5186beb5286ab1)), closes [#558](https://github.com/marvs8/TrustLink/issues/558)
* add require_issuer guard to revoke_attestation ([e63fae9](https://github.com/marvs8/TrustLink/commit/e63fae9160cf62d3e8084752661174bd22d8ced2))
* add require_issuer guard to update_expiration ([5c96fef](https://github.com/marvs8/TrustLink/commit/5c96fefb05c6bd98382b92010d724c581a62b151))
* **anchor-integration:** robust error handling with parseTrustLinkError ([bd13ab3](https://github.com/marvs8/TrustLink/commit/bd13ab38cd657e8dc1e3f267c5b70c8a0dc5536b)), closes [#568](https://github.com/marvs8/TrustLink/issues/568)
* **anchor-integration:** robust error handling with parseTrustLinkError in flow.mjs ([a8807b7](https://github.com/marvs8/TrustLink/commit/a8807b735d5ab2e73978c65d66edbdc03d7a9911))
* **attestations:** prune revoked IDs from subject and issuer indexes ([f842a6f](https://github.com/marvs8/TrustLink/commit/f842a6f80990751e7fe6eccbc6dd6ea5b0fb3a05))
* centralize TTL constants in types.rs, remove raw literals ([#277](https://github.com/marvs8/TrustLink/issues/277)) ([f626eab](https://github.com/marvs8/TrustLink/commit/f626eab3fd1ea8089bf7c2fda67024ccc48183af))
* centralize TTL constants in types.rs, remove raw literals ([#277](https://github.com/marvs8/TrustLink/issues/277)) ([c871bd5](https://github.com/marvs8/TrustLink/commit/c871bd51d4b112115d006a4ac46cc3d323cc2c30))
* enforce CEI pattern in create_attestation, document reentrancy (… ([8c4f491](https://github.com/marvs8/TrustLink/commit/8c4f491ee55fefee73f8ec6bfd955de942ad498b))
* enforce CEI pattern in create_attestation, document reentrancy ([#275](https://github.com/marvs8/TrustLink/issues/275)) ([6377303](https://github.com/marvs8/TrustLink/commit/637730322a15950da4bf1013809cdaa5ebba63a1))
* enforce RateLimitConfig per issuer in create_attestation ([6034d2a](https://github.com/marvs8/TrustLink/commit/6034d2ad2201c5541c6daed1fb98d0c74c321e06))
* enforce RateLimitConfig per issuer in create_attestation ([#282](https://github.com/marvs8/TrustLink/issues/282)) ([aae8f5c](https://github.com/marvs8/TrustLink/commit/aae8f5ccddb5bd0ec4467378aa254f2bab4285ad))
* **errors:** replace raw panics with typed Error variants ([#283](https://github.com/marvs8/TrustLink/issues/283)) ([574bf08](https://github.com/marvs8/TrustLink/commit/574bf085c2dac65909d93c95d6856889ff308138))
* **errors:** replace raw panics with typed Error variants ([#283](https://github.com/marvs8/TrustLink/issues/283)) ([cddf4b6](https://github.com/marvs8/TrustLink/commit/cddf4b6a7d29d4495c0de092e7c58c762396d0f2))
* **errors:** replace raw panics with typed Error variants ([#283](https://github.com/marvs8/TrustLink/issues/283)) ([e7c3094](https://github.com/marvs8/TrustLink/commit/e7c30947cb031b4532466a727c574ae4700d728e))
* **errors:** replace raw panics with typed Error variants ([#283](https://github.com/marvs8/TrustLink/issues/283)) ([#434](https://github.com/marvs8/TrustLink/issues/434)) ([73c9be4](https://github.com/marvs8/TrustLink/commit/73c9be4cdf1f4342b2872532293bfe242c5af55b))
* **events:** emit contract_paused/contract_unpaused events with admin topic ([b4910ff](https://github.com/marvs8/TrustLink/commit/b4910ff0f67226e42925448c9edeacf4e7144a8e))
* **events:** emit contract_paused/contract_unpaused events with admin topic ([557fc44](https://github.com/marvs8/TrustLink/commit/557fc440702a221e8cdd4ef5e7ba055ae457d5f7)), closes [#286](https://github.com/marvs8/TrustLink/issues/286)
* extract store_attestation helper to eliminate duplication ([c20ad05](https://github.com/marvs8/TrustLink/commit/c20ad054d817abdf4ba0b65b08946b14028e9e44))
* filter expired pending requests and add list_delegations_by_dele… ([747e00c](https://github.com/marvs8/TrustLink/commit/747e00c2e4c99dd710269e186359190d34afeccb))
* filter expired pending requests and add list_delegations_by_delegator ([1797e46](https://github.com/marvs8/TrustLink/commit/1797e46dc7ba4910d9b73ae11693250bb1cb2c41))
* fire expiration hook in all claim-check variants and add cancel_… ([d5b8ed6](https://github.com/marvs8/TrustLink/commit/d5b8ed692153af552789fa280054637bd8d5d8c5))
* fire expiration hook in all claim-check variants and add cancel_request ([328178f](https://github.com/marvs8/TrustLink/commit/328178fcf9d33adba38f84c851430db49fd98598))
* freighter-api v6, tsconfig split config, install deps ([872ffe3](https://github.com/marvs8/TrustLink/commit/872ffe347692d437941e566a6e65862fcc9b14f0))
* import ContractConfig in admin.rs ([348b21d](https://github.com/marvs8/TrustLink/commit/348b21d6f04ca22f7feb5f5904ed6db0d62ab8eb))
* **indexer:** multi-stage Dockerfile and GHCR publish workflow ([aac4c50](https://github.com/marvs8/TrustLink/commit/aac4c50a968c02be9f4cc230d4cc2b6da56a460a))
* **indexer:** multi-stage Dockerfile and GHCR publish workflow ([948a8fb](https://github.com/marvs8/TrustLink/commit/948a8fbb9dc68060a04d0da5159bb978603c511b))
* **limits:** enforce storage limits in create_attestation and import_… ([4dc20ad](https://github.com/marvs8/TrustLink/commit/4dc20adb22eadf1883b62f47333561ed8f47d4e1))
* **limits:** enforce storage limits in create_attestation and import_attestation ([fc5e809](https://github.com/marvs8/TrustLink/commit/fc5e80987da5ff05da9745e77f1a05d9354b7fbc)), closes [#318](https://github.com/marvs8/TrustLink/issues/318)
* **makefile:** add verify target and wire deploy reminder ([1d53448](https://github.com/marvs8/TrustLink/commit/1d534482358a4e28aa0baec6c91d3493bd851bfe)), closes [#568](https://github.com/marvs8/TrustLink/issues/568)
* **makefile:** integrate verify_deployment.sh as make verify target ([6654e5d](https://github.com/marvs8/TrustLink/commit/6654e5d3b95c6ed289c852643b928a5333f168b1))
* prevent bridge contracts from being registered as issuers ([ca8a740](https://github.com/marvs8/TrustLink/commit/ca8a74021ad1a13560a6714c3bd707d8163cf5f0))
* prevent bridge contracts from being registered as issuers ([#288](https://github.com/marvs8/TrustLink/issues/288)) ([b0eda6c](https://github.com/marvs8/TrustLink/commit/b0eda6ce89560fcfc41d77ebb15f13ac6ed9001e))
* prevent bridge contracts from being registered as issuers ([#288](https://github.com/marvs8/TrustLink/issues/288)) ([3dfcb6b](https://github.com/marvs8/TrustLink/commit/3dfcb6b16a59fa5d9d4e834de6fdb20096e2f4f1))
* **query:** add cursor-based pagination for get_attestations_in_range ([8e15429](https://github.com/marvs8/TrustLink/commit/8e15429bc31ec2607eb05ab9979e858a900b1565))
* **query:** add cursor-based pagination for get_attestations_in_range and document deletion-safe workflow ([b8ea318](https://github.com/marvs8/TrustLink/commit/b8ea31888eb42e80c8fee6cf35ab227b8d37177b))
* **react-app:** add error boundaries to prevent full app unmount ([8ea2faf](https://github.com/marvs8/TrustLink/commit/8ea2faf4e9422877a2439040c3848314a5d688e7))
* **react-app:** add error boundaries to prevent full app unmount on panel errors ([1fbec92](https://github.com/marvs8/TrustLink/commit/1fbec92bb5ace1ddc7865d0ac19b93b888c38ca7))
* **react-app:** add skeleton loading states for attestation lists ([4406d6f](https://github.com/marvs8/TrustLink/commit/4406d6fb4ed62d686539bb914a046eb5ee59e3fe))
* **react-app:** add skeleton loading states for attestation lists ([18c6d90](https://github.com/marvs8/TrustLink/commit/18c6d90e1d3da016312fd3149af0156c6b78a168))
* remove merge conflict marker from test.rs ([9bb07b1](https://github.com/marvs8/TrustLink/commit/9bb07b1c4c69dd4814df59b767a6ba0faa1583bc))
* remove unused constants and dead code warnings ([540f565](https://github.com/marvs8/TrustLink/commit/540f565e39b7677b827176529d3178d8163469dd))
* resolve 212 compilation errors from duplicate code blocks ([95ceef3](https://github.com/marvs8/TrustLink/commit/95ceef35e5df3eeee66dd537fb539da096af17c4))
* resolve compilation errors in storage, lib, types, and attestation ([5348693](https://github.com/marvs8/TrustLink/commit/53486934404eb6527d8d80f0d2ddaa7289da610d))
* resolve compilation errors in storage, lib, types, and attestation ([196d800](https://github.com/marvs8/TrustLink/commit/196d800ef26ca4598956619c5127309d960b209f))
* resolve issues [#260](https://github.com/marvs8/TrustLink/issues/260), [#327](https://github.com/marvs8/TrustLink/issues/327), [#329](https://github.com/marvs8/TrustLink/issues/329), [#334](https://github.com/marvs8/TrustLink/issues/334) ([75dd029](https://github.com/marvs8/TrustLink/commit/75dd029525b6fc6cee129d17dc2874eaaaa6c0c3))
* resolve issues [#260](https://github.com/marvs8/TrustLink/issues/260), [#327](https://github.com/marvs8/TrustLink/issues/327), [#329](https://github.com/marvs8/TrustLink/issues/329), [#334](https://github.com/marvs8/TrustLink/issues/334) ([0a9151f](https://github.com/marvs8/TrustLink/commit/0a9151f0c1468fb2696122865703c4fb87dc8c56))
* resolve issues [#331](https://github.com/marvs8/TrustLink/issues/331), [#367](https://github.com/marvs8/TrustLink/issues/367), [#368](https://github.com/marvs8/TrustLink/issues/368), [#369](https://github.com/marvs8/TrustLink/issues/369) ([78a5d3f](https://github.com/marvs8/TrustLink/commit/78a5d3f331735f540b174d7466a8a071fec06fe7))
* resolve issues [#331](https://github.com/marvs8/TrustLink/issues/331), [#367](https://github.com/marvs8/TrustLink/issues/367), [#368](https://github.com/marvs8/TrustLink/issues/368), [#369](https://github.com/marvs8/TrustLink/issues/369) ([ca88c6b](https://github.com/marvs8/TrustLink/commit/ca88c6b577fcd39695d63610112adae83f3127ce))
* resolve issues [#522](https://github.com/marvs8/TrustLink/issues/522), [#523](https://github.com/marvs8/TrustLink/issues/523), [#524](https://github.com/marvs8/TrustLink/issues/524), [#525](https://github.com/marvs8/TrustLink/issues/525) ([c382f4a](https://github.com/marvs8/TrustLink/commit/c382f4a21b47e288f9e52e787fc33a77a2eee32c))
* resolve issues [#522](https://github.com/marvs8/TrustLink/issues/522), [#523](https://github.com/marvs8/TrustLink/issues/523), [#524](https://github.com/marvs8/TrustLink/issues/524), [#525](https://github.com/marvs8/TrustLink/issues/525) ([2da81d0](https://github.com/marvs8/TrustLink/commit/2da81d0e12a60c66be68bb8e1ca2708ba9d36717))
* resolve pre-existing compilation errors blocking proptest suite ([ac60fea](https://github.com/marvs8/TrustLink/commit/ac60fea937e0b8a67c7aa7d86da8743012861366))
* restore truncated devcontainer.json ([7e0595f](https://github.com/marvs8/TrustLink/commit/7e0595fbe10e3d1f18bb03739625f7a98c20b5de))
* restore truncated devcontainer.json ([94050a8](https://github.com/marvs8/TrustLink/commit/94050a8c0aeffa355644432f08df57696bbb23fe))
* **search:** implement date-range edge cases and fix variable name bug ([12903da](https://github.com/marvs8/TrustLink/commit/12903daf3b0df51083dc6cd7f16fafa03d3f50c7))
* **security:** complete auth-first audit for all public functions ([#432](https://github.com/marvs8/TrustLink/issues/432)) ([2194e7c](https://github.com/marvs8/TrustLink/commit/2194e7ca93ce136c413bb925ec6891d0bc8c9d67)), closes [#270](https://github.com/marvs8/TrustLink/issues/270)
* split lib.rs into modules and fix pre-existing compile errors ([a55a6b9](https://github.com/marvs8/TrustLink/commit/a55a6b972ec61b8a589bd5445ea8783bf1ec4b1f))
* validate bridge source reference lengths ([37d989c](https://github.com/marvs8/TrustLink/commit/37d989c6830d641eabb80d6d27b8291d5243c76e))
* validate bridge source reference lengths ([9424bba](https://github.com/marvs8/TrustLink/commit/9424bbaf312b03bbad2053b616c247fbcaca7427))
* validate claim_type length and chars in create_attestation ([#278](https://github.com/marvs8/TrustLink/issues/278)) ([465b3a6](https://github.com/marvs8/TrustLink/commit/465b3a61baf1c0e7eb902c0dd80df3e9bdec600f))
* validate claim_type length and chars in create_attestation ([#278](https://github.com/marvs8/TrustLink/issues/278)) ([9680625](https://github.com/marvs8/TrustLink/commit/9680625488f2491963c24d3c366c19604550ba3a))
* validate fee_token implements token interface in set_fee ([#276](https://github.com/marvs8/TrustLink/issues/276)) ([f8900aa](https://github.com/marvs8/TrustLink/commit/f8900aa0b29a2cf479cf84148c6e13b165f494a9))
* validate fee_token implements token interface in set_fee ([#276](https://github.com/marvs8/TrustLink/issues/276)) ([1c04ab7](https://github.com/marvs8/TrustLink/commit/1c04ab7237bb9651d920cb7b72a3b945838c5fd5))
* wire contract pause/unpause to all write operations ([90def6e](https://github.com/marvs8/TrustLink/commit/90def6e1520913c31e5ad1dfec44491f11443142))
* wire contract pause/unpause to all write operations ([#301](https://github.com/marvs8/TrustLink/issues/301)) ([23fd0ca](https://github.com/marvs8/TrustLink/commit/23fd0cadae23e54c441d852fb0d3b82baca04809))


### Performance Improvements

* add wasm-opt -Oz to build pipeline and document size reduction ([c819e60](https://github.com/marvs8/TrustLink/commit/c819e60dc3c7849a59bca0b2e9fbe115efd420f7))
* benchmark and document storage cost per attestation ([7b71e90](https://github.com/marvs8/TrustLink/commit/7b71e90171184e83c5189b8367f85b4d2990e123))
* implement chunked index storage for lazy partial index loading ([dd0fa34](https://github.com/marvs8/TrustLink/commit/dd0fa34fe7dc0e6e035ca6c428147f23c3d34fd1))
* optimize batch attestation to write issuer index once per batch ([790a84a](https://github.com/marvs8/TrustLink/commit/790a84a63d0802eb65b0abc2f43784a1eadb4b96))
* verify has_valid_claim short-circuit and add attestation benchm… ([c6ab909](https://github.com/marvs8/TrustLink/commit/c6ab90901e81708700da0dad3c0936abd08e9ec3))
* verify has_valid_claim short-circuit and add attestation benchmarks ([44cb729](https://github.com/marvs8/TrustLink/commit/44cb72993a467b10a0462100ebb94a77e7c11a20))

## [Unreleased]

<!-- Add new changes here before they are released. Use the categories below:
### Added
### Changed
### Deprecated
### Removed
### Fixed
### Security
-->

## [0.1.0] - 2026-03-25

### Added

- `initialize(admin, ttl_days)` — deploy and set the contract administrator with configurable storage TTL.
- `register_issuer(admin, issuer)` — admin registers a trusted attestation issuer.
- `remove_issuer(admin, issuer)` — admin removes an issuer from the registry.
- `is_issuer(address)` — query whether an address is an authorized issuer.
- `get_admin()` — return the current admin address.
- `transfer_admin(current_admin, new_admin)` — transfer contract administration rights.
- `create_attestation(issuer, subject, claim_type, expiration, metadata)` — issuer creates a new attestation with optional expiration and metadata; returns a deterministic hash-based ID.
- `revoke_attestation(issuer, attestation_id)` — issuer marks an attestation as revoked.
- `get_attestation(attestation_id)` — fetch full attestation data by ID.
- `get_attestation_status(attestation_id)` — return `Valid`, `Expired`, or `Revoked`; emits an `expired` event when status is `Expired`.
- `has_valid_claim(subject, claim_type)` — returns `true` if the subject holds a non-expired, non-revoked attestation of the given type; emits an `expired` event for any expired attestation encountered.
- `has_valid_claim_from_issuer(subject, claim_type, issuer)` — constrain verification to a specific issuer.
- `has_any_claim(subject, claim_types)` and `has_all_claims(subject, claim_types)` — OR/AND claim verification across multiple claim types.
- `get_subject_attestations(subject, start, limit)` — paginated list of attestation IDs for a subject.
- `get_issuer_attestations(issuer, start, limit)` — paginated list of attestation IDs issued by an issuer.
- `get_subject_attestation_count(subject)`, `get_issuer_attestation_count(issuer)`, and `get_valid_claim_count(subject)` — aggregate query helpers.
- Claim type registry: `register_claim_type`, `update_claim_type`, `remove_claim_type`, `get_claim_type_description`, and `list_claim_types`.
- Historical import support: `import_attestation(admin, issuer, subject, claim_type, timestamp, expiration)` and `Attestation.imported`.
- Fee configuration: `set_fee(admin, fee, collector, fee_token)` and `get_fee_config()` with optional token-denominated attestation fees.
- Bridge support: `register_bridge`, `remove_bridge`, `is_bridge`, and `bridge_attestation` with source-chain metadata.
- Batch operations: `create_attestations_batch` and `revoke_attestations_batch`.
- Expiration hooks: `register_expiration_hook`, `get_expiration_hook`, and `remove_expiration_hook` for callback notifications.
- Multi-signature attestations: `propose_attestation`, `cosign_attestation`, and `get_multisig_proposal`.
- Global and per-issuer statistics: `get_global_stats`, `get_issuer_stats`, and issuer tier/metadata management.
- Comprehensive event set for creation, revocation, bridge/import, fee updates, claim-type administration, multi-sig lifecycle, and expiration hooks.
- Integration examples under `examples/` including KYC token and governance-gated voting patterns.

### Fixed

- Validation coverage for metadata, tag cardinality/length, and timestamp/expiration edge cases.
- Deterministic storage/index consistency for issuer and subject attestation lookups.
- Authorization checks across admin, issuer, bridge, and multisig signer flows.

[Unreleased]: https://github.com/Haroldwonder/TrustLink/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Haroldwonder/TrustLink/releases/tag/v0.1.0
