# Alexandria
AtomicCorp.

Alexandria is a system that turns the vast library of YouTube educational content into in depth LaTeX documents to serve the same purpose as a textbook, to educate the user on the topic they want to learn. It takes an aggregate of hundreds to thousands of videos on a given topic - i.e "Intro to Biology" - and turns it into a single LaTeX PDF document that can be used to study. The endgoal is for this to archive YouTube's quality educational content and provide a free source of education for anyone. Solves the problem of "I want to learn about this topic, but there are so many videos to search through, and I don't know where to start." It's name derives from the Library of Alexandria, a great library that was unfortunately lost to time. The program is a standalone executable (not webpage!) with an easy to use and understand GUI that can be run on Windows, Linux, or MacOS, on ARM or x86 architectures.

Pipeline:
Topic input --> Download transcripts for 1000 (or x) videos on that subject that were accepted via a comment rating criteria with NTLK --> 1000 (or x) number of transcripts are summarized in a divide by two tree structure, then reassembled at the end for a coherent output --> Output then parsed into LaTeX by the LLM --> Presented to user.

1. Core infrastructure needed & used:
- Rust -- Core YouTube search with information and transcript retrieval.
- SQLite -- Storage database of transcriptions, and video information for quick successive pull.
- Python -- Parsing / appending to and from stored data.
- JS or Python NiceGUI -- Frontend / Driver for the executable.

2. Supplemental scaffolding:
- NTLK Sentiment Analysis -- Analyze comments to determine if video should be added to database or not - quality control.
- LaTeX -- Foundation of final report presented to user
- a pre-packaged LLM -- Turn data into summarized LaTeX output

3. API routing (FUTURE EXPANSIVE IDEA, NOT TO BE IMPLEMENTED NOW)
- User --> API Key (via Cloudflare) --> LLM API --> User

4. Libraries used
- RustyPipe - https://docs.rs/rustypipe/latest/rustypipe/
- ytx-cli - https://crates.io/crates/ytx-cli


ALREADY DONE:
Core YouTube search and transcript retrieval (MimeographII)
