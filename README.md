# Alexandria
AtomicCorp.



1. Core infrastructure needed & used:
- Rust -- Core YouTube search with information and transcript retrieval
- SQL -- Storage database of transcriptions, and video info
  - Alternatively, a Python Excel library can be used for easier consumer use.
- Python -- Parsing / appending to and from stored data
- JS or Python NiceGUI? -- Frontend / Driver

2. Supplemental scaffolding:
- NTLK Sentiment Analysis -- Analyze comments to determine if video should be added to database
- LaTeX -- Foundation of final report presented to user
- a pre-packaged LLM via Cloudflare routed API -- Turn data into LaTex output

3. API routing
- User --> API Key (via Cloudflare) --> LLM API --> User

5. Libraries used
- RustyPipe - https://docs.rs/rustypipe/latest/rustypipe/
- ytx-cli - https://crates.io/crates/ytx-cli
