# rex_sort

A variety of different sorts in Rust. Choose the right tools for the job, so make tools. Inspired by the idea of "rewrite it in Rust". Starting from the beginning and working my way up. Tried to make them as generic as possible. Will accept any type with partial Ord (and Clone for merge_sort). If you use then make sure any struct has #[derive(PartialOrd, Clone)] I guess. 

All code from wikipedia pseudocode, i.e. "https://en.wikipedia.org/wiki/Quicksort". 

Issues: 

1. dutch_flag_sort is not working. Will figure out later.
2. Implement burst_sort, tim_sort, ska_sort, msd_radix_sort, pdquicksort, smooth_sort, spread_sort, flash_sort, splay_sort
3. Setup tests (nothing is configured with tests, only manual inspection in main)

Eventually, I might make it a lib, but def no rush on that. Hundreds of better sorts out there. Mostly here for other to see and provide feedback. I may also try to optimize when I get time. 
