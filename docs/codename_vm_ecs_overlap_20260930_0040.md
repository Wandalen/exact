# codename_vm_ecs

_Exported 9/30/2026, 12:40:38 AM_

## 🟣 Question

module
For-keeps Rust crates for codename_space_sandbox. A crate lives here once it is proven and meant to be kept, as opposed to spike/, which holds evaluation-stage crates that may be deleted whole. Everything that used to live here (game_core, game_client, slingshot_lab, shaders, verlet_physics, verlet_demo) was reclassified as still-evaluation-stage and moved to spike/ — see ../spike/readme.md.
The nine crates below are the Demiurg stack: an event-driven, persistent computation substrate shaped as an ECS. They are for-keeps rather than spikes because the disposable stage already happened — spike/ecs_orbits_flecs, spike/ecs_orbits_hecs, and spike/ecs_zero_copy are what established that flecs cannot serve the browser target, that hecs can but offers nothing toward zero-copy, and that genuine zero-copy is reachable only by owning the storage layout. The reasoning is recorded in Flecs as Baseline, and the scope they are being built to in Demiurg v0.1. Two of the nine — demiurg_content and demiurg_systems — are build-time compiler libraries invoked from a consumer's own build.rs, not runtime members of the facade chain below; see § Dependency Graph for why they sit outside that chain.
The four crates after that are the Aether stack: a distributed, actor-model computation substrate for entities that need OS-backed copy-on-write snapshots and cross-domain (gaming and enterprise) delivery, sharing the aether prefix as one workstream per task/decisions.md Q-05. Unlike Demiurg, no disposable spike preceded it — its exploration phase was a design conversation external to this repository, transcribed into for-keeps documentation directly; see Aether v0.1's Placement section for that deliberate, acknowledged exception. Why it coexists with Demiurg instead of replacing it is recorded in Aether vs Demiurg.
Two more crates, mpsc_ring and bump_log, are a Shared substrate: family-neutral infrastructure neither stack owns, factored out because Demiurg's intent-submission merge and Aether's actor Mailbox/CommandBuffer need the same underlying mechanisms — a multi-producer ring and a thread-local append log — independently. Neither is demiurg_- nor aether_-prefixed, the same shape this project already uses for cgtools: a shared leaf, no edge between its consumers. Full rationale: task/decisions.md Q-09.
Four more crates are Workstream production homes: exchange_core, temporal_substrate, exact_arithmetic, and transport_layer — one per workstream homed in this repository (002/004/006/007), created per task/decisions.md Q-10 so every such workstream's production crate exists ahead of implementation. All four are true leaves as skeletons; where a workstream has a spike (spike/temporal_substrate, spike/exact_arithmetic), the spike stays its disposable evaluation ground and proven content graduates into the module crate.
Three more families exist because task/021 needs them to smoke-test: building an Asteroids-style minigame demanded an ECS, a rendering engine, and a manifest language stack that did not previously exist in this repository, so that task's own Dependency Inventory is what specifies all sixty-four crates below — the demo is what drives the engine into existence, not the other way around. None of the three families competes with Demiurg or Aether: demiurg/docs/architectural_evaluation/001_flecs_as_baseline.md and aether/docs/architectural_evaluation/001_aether_vs_demiurg.md both stay scoped to persistent-computation and distributed-actor substrates respectively, neither of which is what a real-time single-player minigame's per-frame step loop needs. Neither document discusses the ECS Engine family directly, though — see § Responsibility Overlap Audit below for the crate-level comparison this claim actually rests on.
The ECS Engine stack (22 crates, ecs_*) is a from-scratch, hand-rolled entity-component-system — not flecs_ecs (an off-the-shelf binding proving nothing about this project's own storage design, already rejected once for spike/ecs_orbits_flecs's own reasons) and not Demiurg or Aether (both real substrates, but built for a different problem: durable, multi-process world state, not a single process stepping a game loop at 60 Hz). It is finest-grained on purpose, one crate per micro-concern (ids, pages, arena, columns, archetypes, queries, commands, scheduling, snapshotting), so a consumer needing only ecs_query never compiles the replay or channel machinery it does not use.
The Render Engine stack (32 crates — render_*, rhi_*, shader_*, mesh_*, material_*, visibility_*, light_data, shadow_map, post_chain, vfx_particles, ui_draw, texture_stream, present_*) is a from-scratch 2D/3D rendering engine layered on cgtools's minwgpu module, per the local rulebook's Graphics & Math : cgtools Exclusive rule — rhi_vulkan and rhi_webgl are minwgpu-backed native and wasm targets respectively, not hand-rolled Vulkan/WebGL bindings, so no crate in this family opens a second, independently-versioned graphics surface beside cgtools' own. Sim crates (the ECS family above) never depend on any render_*/rhi_* crate; only render_extract and render_client reach back into the ECS family, to pull a read-only snapshot for drawing.
The Lang Stack (10 crates, lang_*) parses and validates the manifest languages ecs_manifest_load wires into a running world: schema, prototype, system, pipeline, scenario, and material (render-side), plus partition, channel, and fabric for the sharding/multiplayer stages this demo does not yet exercise. Every lang_* crate depends on lang_common only, plus sibling lang_* crates for name resolution (e.g. lang_system checks component names against lang_schema's output) — never on ecs_runtime or any rhi_*/render_* crate; ecs_manifest_load is the only crate permitted to depend on both a lang_* crate and ecs_runtime, because wiring parsed manifests into a live world is its entire purpose.
All eighty-three are skeletons — manifests, crate doc comments, and documentation, with no implementation yet. The sixty-four ecs_*/render_*/rhi_*/lang_* crates additionally carry a documented test surface under tests/docs/ (per l1_imp_surface.rulebook.md), every case marked ⏳ — specified, not yet implemented, the same status Demiurg's and Aether's own future test suites will carry once their implementation tasks land.
Responsibility Table

text
| Directory           | Responsibility                                                                      |
| ------------------- | ----------------------------------------------------------------------------------- |
| demiurg/            | One surface over the six crates below                                               |
| demiurg_arena/      | Pointer-free paged storage and entity Id encoding                                   |
| demiurg_log/        | Append-only record of submitted intent                                              |
| demiurg_query/      | Matching component signatures to archetype tables                                   |
| demiurg_rt/         | The commit loop advancing one Aeon to the next                                      |
| demiurg_schema/     | Component layout, archived beside the data                                          |
| demiurg_store/      | Archetype columns and the Aeons sharing their pages                                 |
| demiurg_content/    | Compiles manifest-authored components/templates into demiurg_schema-conformant Rust |
| demiurg_systems/    | Compiles manifest-authored system wiring into demiurg_rt-conformant Rust            |
| aether/             | One surface over the three crates below                                             |
| aether_page/        | OS-level copy-on-write page pool, native and Wasm backends                          |
| aether_chunk/       | Arrow-RecordBatch-compatible chunked archetype layout                               |
| aether_actor/       | Actor mailboxes, command buffers, tick-phase pipeline                               |
| mpsc_ring/          | Multi-producer ring: many threads publish, one consumer drains in total order       |
| bump_log/           | Per-thread, bump-allocated, zero-lock append log                                    |
| exchange_core/      | Order matching under price-time priority with escrow holds                          |
| temporal_substrate/ | Event calendar and tick scheduling                                                  |
| exact_arithmetic/   | No-float conserved-value types and conservation audit                               |
| transport_layer/    | QUIC/WebTransport sessions, channels, and delta framing                             |
| ecs_id/             | Generational entity ids                                                             |
| ecs_page/           | Pages, dirty tracking, copy-on-write hooks                                          |
| ecs_arena/          | Closed heap, allocation within capacity rules                                       |
| ecs_column/         | SoA column storage and layout                                                       |
| ecs_archetype/      | Tables keyed by component signature                                                 |
| ecs_entity/         | Entity operations surface                                                           |
| ecs_component/      | Component registration                                                              |
| ecs_resource/       | Singleton resources — SimTime, Input, View                                          |
| ecs_query/          | Iteration and filters over archetypes                                               |
| ecs_command/        | Command buffer for structural spawn/despawn ops                                     |
| ecs_relation/       | Relations and cardinality checks                                                    |
| ecs_event/          | Same-frame event buffer                                                             |
| ecs_snapshot/       | Generation freeze and pin for read-only extract                                     |
| ecs_channel/        | Channel manifest and bounded rings, thread/process                                  |
| ecs_system/         | System manifest binding to a Rust body                                              |
| ecs_schedule/       | Slice / readonly / mutator ordering                                                 |
| ecs_pipeline/       | Stages, facts, and sync points                                                      |
| ecs_host/           | HostRef and incoming/outgoing boundary                                              |
| ecs_step/           | The step() driver                                                                   |
| ecs_runtime/        | Wires the above into one World                                                      |
| ecs_manifest_load/  | Loads all lang_* output into a runtime                                              |
| ecs_replay/         | Record / playback checksums                                                         |
| render_types/       | MeshId, MaterialId, View, and other handles                                         |
| render_math/        | Render-local math built on ndarray_cg                                               |
| render_color/       | Color types and conversions                                                         |
| rhi_api/            | Render hardware interface — devices, queues, pipelines                              |
| rhi_null/           | Headless RHI backend for CI and tests                                               |
| rhi_vulkan/         | Native RHI backend, minwgpu-based                                                   |
| rhi_webgl/          | Wasm RHI backend, minwgpu-based                                                     |
| render_pass/        | One render pass description                                                         |
| render_graph/       | Passes, targets, barriers compiled to RHI                                           |
| shader_ir/          | Shader intermediate representation                                                  |
| shader_compiler/    | Compiles shader IR for a backend                                                    |
| shader_cache/       | Compiled shader permutation cache                                                   |
| mesh_data/          | CPU-side mesh geometry                                                              |
| mesh_gpu/           | GPU-uploaded mesh buffers                                                           |
| material_data/      | Material definitions                                                                |
| material_instance/  | Bound material parameters and batching keys                                         |
| visibility_cull/    | Frustum culling                                                                     |
| visibility_lod/     | Level-of-detail selection                                                           |
| render_extract/     | Read-only ECS snapshot to draw packets                                              |
| render_draw_list/   | Draw list assembled from extracted packets                                          |
| render_batch/       | Groups draws by material/batching key                                               |
| render_submit/      | Sorts and submits batches into the render graph                                     |
| light_data/         | Light source data                                                                   |
| shadow_map/         | Shadow map pass                                                                     |
| post_chain/         | Post-processing pass chain                                                          |
| vfx_particles/      | Particle effects                                                                    |
| ui_draw/            | HUD/UI draw pass                                                                    |
| render_asset_load/  | Loads mesh/material/shader assets                                                   |
| texture_stream/     | Texture streaming and mip management                                                |
| present_swapchain/  | Native swapchain presentation                                                       |
| present_canvas/     | Canvas/wasm presentation                                                            |
| render_client/      | One frame: extract → graph → present                                                |
| lang_common/        | Shared tokens, spans, diagnostics, component paths                                  |
| lang_schema/        | Parses/validates schema manifests (type/component/resource)                         |
| lang_prototype/     | Parses/validates prototype manifests                                                |
| lang_system/        | Parses/validates system manifests                                                   |
| lang_pipeline/      | Parses/validates pipeline manifests                                                 |
| lang_scenario/      | Parses/validates scenario (world bootstrap) manifests                               |
| lang_material/      | Parses/validates material (render pass binding) manifests                           |
| lang_partition/     | Parses/validates partition (sharding) manifests                                     |
| lang_channel/       | Parses/validates channel (transport) manifests                                      |
| lang_fabric/        | Parses/validates fabric (listen/discover/mtu) manifests                             |

Dependency Graph
Leaf-first, per Architecture : Leaf-Proximate Placement in the local rulebook:

text
demiurg_arena   demiurg_schema        (leaves — no demiurg dependencies)
      │    ╲       ╱    │
      │     demiurg_store             (arena + schema)
      │           │
      │      demiurg_query            (store)
      │
 demiurg_log                          (arena + schema)
      ╲          │
       demiurg_rt                     (store + query + log + schema)
             │
         demiurg                      (facade over all six)

aether_page                           (leaf — no aether dependencies)
      │
 aether_chunk                         (page)
      │
 aether_actor                         (chunk)
      │
    aether                            (facade over all three)

One Demiurg edge is declared by design but not yet in the manifest: demiurg_rt takes demiurg_schema as a direct dependency at its first implementation increment — the schedule digest calls schema's own hash rather than reimplementing it (→ task/019); the skeleton manifest today declares only the other three edges.
One edge crosses the stack boundary, and it is in the manifest today: aether_chunk takes a direct dependency on demiurg_schema, the same shape of edge demiurg_content already has — component-width vocabulary reuse, not storage. It's the ASCII diagram's one deliberate omission: drawing it would visually connect two boxes that are otherwise correctly separate, so it's recorded here as a footnote instead of redrawn into the graph (→ task/decisions.md Q-08 for the decision; Aether vs Demiurg for why the storage-layer independence the diagram's separation still depicts is untouched).
The two stacks are independent at the storage/allocator layer: no demiurg* crate depends on an aether* crate or vice versa there. See Aether vs Demiurg for why that holds regardless of the one vocabulary-level exception above.
Two more crates sit entirely outside this graph, touching neither stack. mpsc_ring and bump_log are shared leaves with no dependencies and, as of this decision, no dependents either — neither demiurg_log nor aether_actor has been wired to them yet. Real adoption is future work gated on docs/workstream/008_concurrency_benchmarks.md's verdict (→ task/decisions.md Q-09); until then they exist as skeletons only, proving out the family-neutral shape without touching either stack's current dependency edges.
Four more crates sit outside it the same way. The workstream production homes exchange_core, temporal_substrate, exact_arithmetic, and transport_layer (→ task/decisions.md Q-10) are leaves with no dependents. Their designed edges — exchange_core hosting on demiurg's facade (Q-07) and consuming exact_arithmetic, demiurg_rt driving temporal_substrate, transport_layer carrying aether_chunk's frozen wire format as a byte contract rather than a Cargo edge — are recorded in their workstream instances (docs/workstream/) and land in manifests at each crate's first implementation increment.
Two Demiurg crates sit mostly outside this graph. demiurg_content and demiurg_systems are build-time compiler libraries invoked from a consumer's build.rs, not runtime crates any facade-chain member depends on. They are not symmetric with each other, though: demiurg_content takes a real [dependencies] edge on demiurg_schema so its own code can call demiurg_schema's primitive-kind admissibility check directly instead of reimplementing that closed, evolving vocabulary (→ demiurg_content's algorithm/001's "Why a Library Call" section); demiurg_systems has no such edge, because the one check its own Step 3 performs — reads/writes are subsets of query — is pure set membership over manifest-local, freeform names, with no closed vocabulary owned by demiurg_rt for it to drift out of sync with (→ demiurg_systems's algorithm/001). What either crate generates also targets another crate's conformance in its output, which is a codegen pairing a future consumer must declare in its own manifest, separately from either crate's own dependency edge:

text
demiurg_content   real dep: demiurg_schema (admissibility check)
                  generates demiurg_schema-conformant code   → a future consumer pairs
                                                                 [build-dependencies] demiurg_content
                                                                 with [dependencies] demiurg_schema

demiurg_systems   real dep: none (true leaf)
                  generates demiurg_rt-conformant code        → a future consumer pairs
                                                                 [build-dependencies] demiurg_systems
                                                                 with [dependencies] demiurg_rt + demiurg_query

Getting the consumer-side split wrong is common enough that each crate documents it directly — see demiurg_content's pitfall and demiurg_systems's pitfall.
ECS Engine and Render Engine are each too wide (22 and 32 crates) for an ASCII diagram to stay readable — their edges are given as the leveled tables under § Recommended Implementation Order below instead, which is exact where a hand-drawn tree would either omit edges or become unreadable. Two invariants hold across both graphs regardless: ecs_* never depends on render_*/rhi_* (sim must run headless, on a server with no GPU); and within Render Engine, only render_extract/render_client cross back into ecs_* — every other render crate (RHI, shaders, meshes, materials, graph, visibility, batch/submit, lighting, post, vfx, ui, assets, present backends) stays free of ecs_* so headless tools can use the RHI without a world.
Lang Stack is flat rather than deep — lang_common is the one shared leaf, and every other lang_* crate depends on it plus whichever sibling lang_* crates it needs for name resolution:

text
lang_common                                            (leaf)
   ├─ lang_schema                                       (common)
   ├─ lang_prototype                                     (common + schema — validates `with:`/`set:` against schema names)
   ├─ lang_system                                        (common + schema — validates read/write targets against schema names)
   ├─ lang_pipeline                                       (common — stage/fact names only, no schema dependency)
   ├─ lang_scenario                                       (common + schema + prototype — validates `from: prototype:` references)
   ├─ lang_material                                       (common — render-side; no schema, no ecs_runtime dependency)
   ├─ lang_partition                                      (common — deferred scope, sharding)
   ├─ lang_channel                                        (common — deferred scope, transport framing)
   └─ lang_fabric                                         (common — deferred scope, listen/discover/mtu)

No lang_* crate depends on ecs_runtime or any rhi_*/render_* crate — parsing and validating a manifest never requires a live world or a GPU. ecs_manifest_load is the sole crate depending on both sides, exactly as render_extract/render_client are the sole Render Engine crates depending on ecs_* — three deliberate, narrow bridge crates, everything else on either side stays single-purpose.
Recommended Implementation Order
Derived from the graph above — a crate is buildable once every crate below it in its own stack has landed; crates on the same level have no dependency on each other and are parallel-safe.
Demiurg (7 crates; every one already has a filed, Tier-2-verified task — task/014 through 020):

text
| Level | Crates                        | Gated on                                                     | Task     |
| ----- | ----------------------------- | ------------------------------------------------------------ | -------- |
| 0     | demiurg_arena, demiurg_schema | —                                                            | 014, 015 |
| 1     | demiurg_store, demiurg_log    | Level 0                                                      | 016, 018 |
| 2     | demiurg_query                 | demiurg_store                                                | 017      |
| 3     | demiurg_rt                    | demiurg_store + demiurg_query + demiurg_log + demiurg_schema | 019      |
| 4     | demiurg (facade)              | all six above                                                | 020      |

The 014→020 task numbering reads as one straight line but isn't the real order: demiurg_log (018) depends only on Level 0, so it's buildable alongside demiurg_store (016) rather than waiting on demiurg_query (017) — worth not being misled by the numbers.
Demiurg compiler crates (demiurg_content, demiurg_systems; 2 crates, no task files exist for either yet): not part of the level table above — nothing in the facade chain depends on them, so "buildable once every crate below it has landed" doesn't apply to them as a pair. They aren't symmetric with each other, though: demiurg_systems is a true zero-Cargo-dependency leaf, buildable at any time; demiurg_content has a real dependency on demiurg_schema (Level 0), so it's gated on Level 0 landing the same as any other Level-0-dependent crate, despite sitting outside the facade chain itself (→ § Dependency Graph above for why the two crates diverge here):

text
| Crate           | Real Cargo dependency    | Generated-code target (a future consumer's paired dependency, not this crate's own) | Task       |
| --------------- | ------------------------ | ----------------------------------------------------------------------------------- | ---------- |
| demiurg_systems | none (leaf)              | demiurg_rt + demiurg_query                                                          | none filed |
| demiurg_content | demiurg_schema (Level 0) | demiurg_schema                                                                      | none filed |

Aether (4 crates; no task files exist for any of them yet — filing them, the same way 014–020 were filed for Demiurg, is the prerequisite before implementation can start):

text
| Level | Crate           | Gated on                                  | Task       |
| ----- | --------------- | ----------------------------------------- | ---------- |
| 0     | aether_page     | —                                         | none filed |
| 1     | aether_chunk    | aether_page + demiurg_schema (Demiurg L0) | none filed |
| 2     | aether_actor    | aether_chunk                              | none filed |
| 3     | aether (facade) | all three above                           | none filed |

Shared crates (mpsc_ring, bump_log; 2 crates, no task files exist for either yet): both true leaves, buildable at any time — neither depends on anything, and as of this decision nothing depends on them yet either. Adoption by demiurg_log/aether_actor is future work pending workstream 008's verdict on which concurrency pattern to implement, per task/decisions.md Q-09.
Workstream production homes (exchange_core, temporal_substrate, exact_arithmetic, transport_layer; 4 crates, no task files exist for any yet): all true leaves, buildable at any time as crates. Their implementation order is workstream-grain, not crate-grain — temporal_substrate and exact_arithmetic are first echelon, exchange_core and transport_layer second, per docs/workstream/readme.md § Recommended Build Order — and exchange_core is additionally gated on Demiurg's thread-local-buffer-plus-merge capability landing (Q-07), while 004/006 fill from their spikes' evaluations graduating.
Across the two stacks: parallel-safe with one exception — aether_chunk (and transitively aether_actor, aether) is gated on Demiurg's Level 0 landing (demiurg_schema), per task/decisions.md Q-08. Nothing in Demiurg gates Aether. Neither this file nor task/decisions.md Q-05 states a priority between the two stacks' own internal work; Q-05's own verdict treats both as co-equal independent substrates for everything except this one narrow, later-decided vocabulary edge.
This is one level down from the coarser workstream grain, where Demiurg is one instance among thirteen and Aether has none (see docs/workstream/readme.md for why). The cross-workstream order — e.g. Demiurg relative to Exact Arithmetic or the Temporal Substrate — is already recorded there under Recommended Build Order; the tables above are the order inside the Demiurg and Aether crate families themselves, while the Shared and Workstream leaves have no internal order to record.
ECS Engine (22 crates; driven by task/021, which is the demo whose stage-1 closure these crates are). Each crate now carries its own implementation task in its Local Task System at <crate>/task/, filed under IDs 022–043 by the root Global ID Registry; the Task column below names them:

text
| Level | Crates                                           | Gated on                                                                                      | Task               |
| ----- | ------------------------------------------------ | --------------------------------------------------------------------------------------------- | ------------------ |
| 0     | ecs_id                                           | —                                                                                             | 022                |
| 1     | ecs_page, ecs_component, ecs_resource, ecs_host  | Level 0                                                                                       | 023, 028, 029, 039 |
| 2     | ecs_arena                                        | ecs_page                                                                                      | 024                |
| 3     | ecs_column                                       | ecs_arena                                                                                     | 025                |
| 4     | ecs_archetype, ecs_relation                      | ecs_column (archetype); ecs_component (relation)                                              | 026, 032           |
| 5     | ecs_entity, ecs_event, ecs_snapshot, ecs_channel | ecs_archetype (entity); ecs_resource (event); ecs_page+ecs_arena (snapshot); ecs_id (channel) | 027, 033, 034, 035 |
| 6     | ecs_query                                        | ecs_archetype + ecs_column + ecs_component                                                    | 030                |
| 7     | ecs_command, ecs_system                          | ecs_archetype + ecs_component (command); ecs_query + ecs_component (system)                   | 031, 036           |
| 8     | ecs_schedule, ecs_replay                         | ecs_system (schedule); ecs_snapshot (replay)                                                  | 037, 043           |
| 9     | ecs_pipeline                                     | ecs_schedule + ecs_system                                                                     | 038                |
| 10    | ecs_step                                         | ecs_schedule + ecs_pipeline                                                                   | 040                |
| 11    | ecs_runtime                                      | every crate in Levels 0–10                                                                    | 041                |
| 12    | ecs_manifest_load                                | ecs_runtime + every lang_* crate (Lang Stack, below)                                          | 042                |

Level is a sequencing position, not a Cargo edge — and the two genuinely differ in three places. A crate's Level says when it is sensible to build it; its task's blocked_by names only the crates it actually [dependencies]-depends on, which is a subset. Three crates sit above their own dependency depth on purpose, each with the reasoning already recorded in its own docs rather than here:

ecs_resource (Level 1, blocked_by null) — no Cargo edge on ecs_id; its storage is generic and opaque over the resource type, so EntityId never enters scope. See ecs_resource/docs/decisions/001_no_ecs_id_cargo_dependency.md.
ecs_host (Level 1, blocked_by null) — same shape: a HostRef is a bare u32 and names nothing. See ecs_host/docs/readme.md and task 039 § Out of Scope.
ecs_replay (Level 8, blocked_by 034 only) — needs ecs_snapshot (Level 5) alone, so it is buildable from Level 6 onward; it is sequenced at 8 because it is only useful once there is a step loop to record, and it shares that level with ecs_schedule rather than gating on it. Its own readme.md and docs/readme.md are the authority here.
Everywhere else the two coincide. Each task's blocked_by was generated from that crate's own [dependencies] and cross-checked against it, so it is the manifest graph verbatim — read blocked_by when you want to know what must compile first, and Level when you want to know what to pick up next.
Staging. Task 021 § Build order splits this family in two, and the task IDs follow that split. Stage 1 is all 22 of the above except 042 — the 21-crate closure of ecs_runtime, which is exactly what spike/asteroids_core needs to compile and what carries every row of 021's Test Matrix but the render ones. Stage 2 is the Render Engine table below. ecs_manifest_load (042) is in neither: it sits above the ecs_runtime facade rather than beneath it, and is reached only if manifests load from YAML — which is why 021's blocked_by names 022–041 and 043 but not 042.
Render Engine (32 crates; same driving task):
text
| Level | Crates                                                                                                               | Gated on                                                                                                                                                               | Task |
| ----- | -------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---- |
| 0     | render_types, render_math, render_color                                                                              | —                                                                                                                                                                      | 021  |
| 1     | rhi_api, mesh_data, visibility_cull, light_data                                                                      | Level 0                                                                                                                                                                | 021  |
| 2     | rhi_null, rhi_vulkan, rhi_webgl, shader_ir, render_pass, mesh_gpu, visibility_lod, present_swapchain, present_canvas | rhi_api (all); mesh_data also (mesh_gpu, visibility_lod)                                                                                                               | 021  |
| 3     | shader_compiler, material_data, render_graph                                                                         | shader_ir (compiler, material_data); render_pass (graph)                                                                                                               | 021  |
| 4     | shader_cache, material_instance, shadow_map, ui_draw, render_asset_load                                              | shader_compiler+rhi_api (cache); material_data+rhi_api (instance); light_data+render_graph (shadow); render_graph (ui); mesh_data+material_data+shader_ir (asset_load) | 021  |
| 5     | render_draw_list, post_chain, vfx_particles, texture_stream                                                          | material_instance+mesh_data (draw_list); shader_cache+render_graph (post_chain, vfx_particles); render_asset_load (texture_stream)                                     | 021  |
| 6     | render_batch, render_extract                                                                                         | render_draw_list+material_instance (batch); render_draw_list + ECS Level 5 ecs_snapshot/ecs_query/ecs_id/ecs_component (extract)                                       | 021  |
| 7     | render_submit                                                                                                        | render_batch+render_graph                                                                                                                                              | 021  |
| 8     | render_client                                                                                                        | render_submit+render_extract+one of rhi_vulkan/rhi_webgl/rhi_null + ECS Level 11 ecs_runtime                                                                           | 021  |

Two crates cross into the ECS Engine table above — render_extract (Level 6) and render_client (Level 8) — the same shape as Aether's one cross-stack edge into Demiurg: drawn as a footnote rather than folded into a combined graph, because the two families stay independent everywhere else. Every other Render Engine crate compiles with zero ecs_* dependencies, so a headless build (server, CI) can exercise the whole RHI/graph/material stack without a world.
Lang Stack (10 crates; same driving task):

text
| Level | Crates                                                                               | Gated on                                   | Task |
| ----- | ------------------------------------------------------------------------------------ | ------------------------------------------ | ---- |
| 0     | lang_common                                                                          | —                                          | 021  |
| 1     | lang_schema, lang_pipeline, lang_material, lang_partition, lang_channel, lang_fabric | lang_common                                | 021  |
| 2     | lang_prototype, lang_system                                                          | lang_common + lang_schema                  | 021  |
| 3     | lang_scenario                                                                        | lang_common + lang_schema + lang_prototype | 021  |

ecs_manifest_load (ECS Engine Level 12) is the only crate depending on the Lang Stack at all — every other ecs_*/render_*/rhi_* crate stays free of lang_*, the same one-bridge-crate shape as render_extract/render_client on the render side.
Across all five families (Demiurg, Aether, Shared/Workstream, ECS Engine, Render Engine, Lang Stack): parallel-safe except for the three named cross-family edges above (aether_chunk→demiurg_schema, render_extract/render_client→ECS Engine, ecs_manifest_load→Lang Stack). Nothing in the two new engine families gates Demiurg or Aether, and nothing in Demiurg or Aether gates them back.
Responsibility Overlap Audit
A cross-crate audit run against every crate in § Responsibility Table above (2026-08-26), looking for pairs whose stated responsibility competes rather than complements, found four pairs worth recording. Only one is a confirmed non-issue; the rest are either already governed by an explicit decision elsewhere or are open questions this table exists to track until one of the crates' consumers forces a resolution.

text
| Pair                                | Crates                                                                                                                                                            | Status                       | Evidence                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| ----------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Shared-substrate migration          | mpsc_ring/bump_log vs aether_actor's Mailbox/CommandBuffer, and vs demiurg_log's intent-submission serialisation point                                            | Tracked, gated               | Both sides cross-reference each other as "prospective consumer"/"candidate replacement" (aether_actor/docs/readme.md, bump_log/docs/readme.md, mpsc_ring/docs/readme.md); the swap is gated on beating the working baseline per each crate's own non_functional_requirement/001_measured_before_adopted.md and docs/workstream/008_concurrency_benchmarks.md — Q-09. No action pending 008's verdict.                                                                                                                                                                                                           |
| ecs_channel vs mpsc_ring            | ecs_channel, mpsc_ring                                                                                                                                            | Resolved, not competing      | ecs_channel/readme.md explicitly distinguishes its same-thread, same-tick single-writer/single-reader ring from mpsc_ring's cross-thread MPSC, citing the local rulebook's Concurrency: Substrate-Hosted, Not Bespoke rule.                                                                                                                                                                                                                                                                                                                                                                                     |
| Demiurg family vs ECS Engine family | demiurg_arena/ecs_id, demiurg_store/ecs_archetype+ecs_column, demiurg_query/ecs_query, demiurg_schema/ecs_component+lang_schema, demiurg_rt/ecs_step+ecs_schedule | Documentation gap, open      | The "None of the three families competes with Demiurg or Aether" claim above cites demiurg/docs/architectural_evaluation/001_flecs_as_baseline.md and aether/docs/architectural_evaluation/001_aether_vs_demiurg.md — neither document mentions the ECS Engine family (verified: no match for ecs_engine/ecs_archetype/ecs_store/render engine in either file's text). The underlying reasoning (durable multi-process world state vs. a 60 Hz single-process game loop) is asserted inline above but has no dedicated architectural_evaluation instance of its own comparing the two families crate-for-crate. |
| Paged-storage/CoW vocabulary echo   | ecs_page, demiurg_arena, aether_page                                                                                                                              | Open, lower confidence       | Zero cross-references found between the three crates' full docs/ trees in any direction (grep-verified pairwise both ways). Each occupies a different layer — OS-level CoW page pool (aether_page), pointer-free arena with entity Id encoding (demiurg_arena), ECS dirty-tracking pages (ecs_page) — but none states this against the other two the way ecs_channel states its distinction against mpsc_ring.                                                                                                                                                                                                  |
| "schema" vocabulary reuse           | demiurg_schema, lang_schema                                                                                                                                       | Naming only, not duplication | Different jobs in different stacks (runtime component layout vs. manifest-language parsing) — flagged only because the bare word is shared across both.                                                                                                                                                                                                                                                                                                                                                                                                                                                         |

The two "open" rows are not a decision to extract shared crates, write a dedicated eval doc, or leave things as documented — that call, once made, belongs in task/decisions.md as a new Q- entry, the same way Q-08/Q-09 already settled the analogous Aether/Shared-substrate questions. This table exists to keep the audit findings from living only in a chat transcript until that decision is made.

we need to handle somehow responsibility overlap! we don't need duplications of crats. we dont need crate just in case. we want avoid YAGNI mistakes. and we want single responsibility per crate and zero overlap. please analyze and tell which we preserve and which we delete. explain why for each and its final responsibility

---

## 🟢 Answer

Worked for 44s

You have three entity machines (Demiurg, Aether, ecs_*) and a render kitchen sink. That is the overlap. Zero-overlap means one storage, one command merge, one page heap, one language compiler, one thin render path.

Rule
One owner per mechanism.
If Asteroids does not need it this year → spike or delete, not module/.
Shared leaves have no family prefix.
Pages / arena / CoW — one heap
Crate	Verdict	Why
aether_page	Keep as mem_page	Only crate that is OS CoW (native + Wasm). That is the mechanism.
demiurg_arena	Keep, shrink	Id encoding + pointer-free placement on mem_page. Stop owning a second page pool.
ecs_page	Delete	Same pages + dirty bits.
ecs_arena	Delete	Same heap.

Final: mem_page = bytes, pin, CoW. demiurg_arena = entity ids in those pages.

Columns / query / step — one ECS

Demiurg is the owned layout (Flecs/hecs spikes already decided that). ecs_* is a second Flecs.

Crate	Verdict	Why
demiurg_schema	Keep	Runtime layout vocabulary.
demiurg_store	Keep	Archetype columns + aeons/pages.
demiurg_query	Keep	Signature → tables.
demiurg_rt	Keep	Commit / tick loop. Aeon = game tick.
demiurg facade	Keep	One surface.
ecs_id	Delete	Ids live in demiurg_arena.
ecs_column	Delete	demiurg_store.
ecs_archetype	Delete	demiurg_store.
ecs_entity	Delete	Facade of store + arena.
ecs_component	Delete	demiurg_schema.
ecs_query	Delete	demiurg_query.
ecs_step	Delete	demiurg_rt.
ecs_schedule	Delete	Scheduling inside demiurg_rt (or one demiurg_schedule if rt gets fat—not a second engine).
ecs_runtime	Delete	demiurg.

Asteroids 60 Hz is a rate on demiurg_rt, not a new family.

Commands / logs / rings — one merge path
Crate	Verdict	Why
mpsc_ring	Keep (008 leaf)	Cross-thread MPSC.
bump_log	Keep (008 leaf)	TLS append.
demiurg_log	Keep as adapter	Intent record on bump_log + drain into rt. Not a second ring.
Aether Mailbox internals	Delete implementation	Same merge; call bump_log / mpsc_ring after 008 bench.
ecs_command	Delete	Same buffer as log/mailbox.
ecs_channel	Keep tiny or fold	Same-thread SPSC only. If it is just Vec per tick → one type in demiurg_rt, not a crate. If you keep a crate: no ring implementation.
ecs_event	Delete as crate	Event queue = bump_log or a Queue<T> in rt.
Aether remainder
Crate	Verdict	Why
aether_page → mem_page	Keep	See above.
aether_chunk	Spike or later	Arrow batches = wire/analytics, not 60 Hz Asteroids. YAGNI in module/ now.
aether_actor	Spike / 013 later	Distributed actors ≠ minigame systems.
aether facade	Delete until chunk+actor exist	Facade over one renamed page crate is noise.
Manifest compilers — one stack
Crate	Verdict	Why
lang_* (common, schema, prototype, system, pipeline, scenario)	Keep	Source of IR.
lang_material	Keep	Render manifests.
lang_partition / channel / fabric	Spike	No multiplayer in demo. YAGNI in module/.
demiurg_content	Keep as build.rs	IR → Rust layout. Consumes lang_schema IR, does not parse a second language.
demiurg_systems	Keep as build.rs	IR → rt wiring. Consumes lang_system / lang_pipeline.
ecs_manifest_load	Rename demiurg_manifest_load or fold into facade	Only runtime bind. Do not keep two loaders.
lang_schema vs demiurg_schema	Both keep	Text vs runtime layout. Document: manifest schema vs layout schema.
ECS leftovers that are real jobs (not a second store)

Add into Demiurg, do not keep 22 crates:

Job	Where it lives
Resources (SimTime, Input)	demiurg_store singleton table or demiurg_resource one crate
Snapshot pin for extract	mem_page pin + demiurg_store freeze
Relations / markers	demiurg_schema + store
Host boundary	demiurg_host one crate (or ecs_host renamed)
Replay	demiurg_replay one crate
Pipeline facts	demiurg_rt or demiurg_pipeline one crate

Do not keep ecs_snapshot + ecs_relation + ecs_resource as a parallel universe—move the tests, drop the ecs_ store.

Workstream leaves — keep (no overlap)

exchange_core, temporal_substrate, exact_arithmetic, transport_layer

They are not a third ECS.

Render — keep path, cut YAGNI

Keep (Asteroids + headless):

render_types, render_math, render_color, rhi_api, rhi_null, rhi_vulkan, rhi_webgl, render_pass, render_graph, shader_ir, shader_compiler, shader_cache, mesh_data, mesh_gpu, material_data, material_instance, render_extract, render_draw_list, render_batch, render_submit, ui_draw, present_swapchain, present_canvas, render_client

Delete or spike (not a minigame):

visibility_lod, light_data, shadow_map, post_chain, vfx_particles, texture_stream

Optional: fold visibility_cull into extract; fold render_asset_load into mesh/material until you have files.

render_extract reads demiurg snapshot, not ecs_snapshot.

What you tell the audit
Overlap C is closed by deleting the ECS Engine store, not by another eval PDF.
Overlap pages is closed by mem_page.
Overlap A stays gated on 008, then Mailbox/log call the shared crates.
B stays: no second MPSC in ecs_channel.
Schema word: two jobs, two crates, two phrases in READMEs.
One sentence

Demiurg + mem_page + bump_log/mpsc_ring + lang_* + a short render path.

Everything that reimplements ids, pages, columns, query, step, or mailbox is deleted, not “documented as complementary.”

---

## 🟣 Question

ecs_system is the one ecs_* crate with no verdict in the ruling — it's absent from the Columns/query/step table and from the leftovers table. What happens to it?
❯ 1. Delete → demiurg_rt (Recommended)
     System binding/registration is the rt's job — the old design had ecs_runtime's register() forward into ecs_system's binding pipeline, which is exactly the 'second engine' the ruling collapses. Consistent with ecs_step and ecs_schedule both going to demiurg_rt.
  2. Keep as demiurg_system
     Own crate for system descriptors, access sets, and manifest binding — parallel to demiurg_query. Justified if rt would otherwise get fat.
  3. Spike it
     Move to spike/ — not needed for Asteroids this year, revisit when manifest-driven systems are real.

---

## 🟢 Answer

Worked for 10s

1. Delete ecs_system. Do not keep it. Do not add demiurg_system.

Why not 2

demiurg_systems (plural) already exists as build.rs codegen. A second crate named demiurg_system is another vocabulary echo and a second engine surface for “what is a system.”

Access sets / hints are data on the rt schedule, not a family.

Why not 3

Asteroids must run integrate_ship, wrap_positions, …. That is not YAGNI. What is YAGNI is a 22nd ECS crate whose only job is register(name, fn).

Why 1

Same collapse as ecs_step / ecs_schedule:

demiurg_rt: register(name, body, AccessSet), run in the aeon/tick.
demiurg_systems: optional later — emit that register from lang_system IR.
Until manifests are live: hand-register Rust fns on the facade. Enough for the demo.

ecs_runtime::register → ecs_system was the duplicate pipeline. It dies with the crate.

If rt gets fat later

Extract then, one crate, e.g. demiurg_bind — not now, and never ecs_system.

Verdict: option 1.

---

## 🟣 Question

The ruling says "spike or delete" for the YAGNI set (visibility_lod, light_data, shadow_map, post_chain, vfx_particles, texture_stream, aether_chunk, aether_actor, lang_partition, lang_channel, lang_fabric) without picking. Which?
  1. Delete all (Recommended)
     rm from module/ entirely. They're committed, so git has them if ever wanted. Cleanest reading of 'not module/' — nothing lingering to re-grow. ~11 crates plus their docs/ and task/ trees.
  2. Spike all
     git mv to spike/ — keeps the docs corpus browsable without it counting as module surface. Costs a spike/ directory that accumulates dead weight.
  3. Split: render deleted, aether+lang spiked
     The 6 render crates are pure YAGNI for a minigame → delete. aether_chunk/actor and lang_partition/channel/fabric have real future intent (Arrow batches, distributed actors, multiplayer) → spike.
❯ 4. Type somet

---

## 🟢 Answer

Worked for 8s

3.

Delete from module/: visibility_lod, light_data, shadow_map, post_chain, vfx_particles, texture_stream.
No Asteroids path, no separate workstream home, no eval that justifies keeping a skeleton in the production tree. Git already remembers them.
git mv → spike/: aether_chunk, aether_actor, lang_partition, lang_channel, lang_fabric.
Those have named future jobs (Arrow wire, actors, shard/channel/fabric langs) and existing design/Q-docs. Spike = “not production, still readable.” Do not leave an aether facade in module/ until those two come back.

Do not spike the six render crates unless their docs/ are the only copy of a decision you care about—if so, move only the markdown under docs/, not a fake crate.

Not 1: you throw away the Aether/lang paper trail that Q-05/multiplayer already paid for.

Not 2: spike/ becomes a junk drawer for lights and LOD you will not open.

---
