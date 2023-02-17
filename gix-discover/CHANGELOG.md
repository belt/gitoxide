# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.13.1 (2023-02-17)

### Bug Fixes

 - <csr-id-e14dc7d475373d2c266e84ff8f1826c68a34ab92/> note that crates have been renamed from `git-*` to `gix-*`.
   This also means that the `git-*` prefixed crates of the `gitoxide` project
   are effectively unmaintained.
   Use the crates with the `gix-*` prefix instead.
   
   If you were using `git-repository`, then `gix` is its substitute.
 - <csr-id-07cd4a869d22476f812e4dafc847894ddebff5a8/> `GIT_CEILING_DIRECTORIES` now handles UNC paths correctly #(730).
   Previously ceiling directories that were UNC paths would not work together
   with directories which were not due to different bases.
   
   Now ceiling directories will be converted non-UNC paths before using them.

## 0.13.0 (2023-02-17)

<csr-id-7057ad2251f76756a5b35b6bbba10ab9b6601226/>
<csr-id-08d0c27b0641526aa8b70ee5aadd3658862bef1b/>
<csr-id-6106521581029c5c24b23a47bb91c1921edfa0af/>
<csr-id-136eb37b00c9e7ba0fd0bc2a75dee2ac1b06516d/>
<csr-id-277c41f1185398e14f92247aada8422f6b08afd1/>
<csr-id-e63e722791a7795cd99048bed834459595c60abc/>
<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

### New Features (BREAKING)

 - <csr-id-b4dcfc716a80ffccbab6f7ccc586d8063cc10fff/> `Path::from_dot_git_dir()` now takes the `current_dir` as argument and returns `Option<path>`
   That way it's possible to avoid at least one call of
   `std::env::current_dir()` per invocation, which also is more consnstent
   with similar plumbing methods.
   
   Furthermore it can signal with `None` if the input directory was invalid.
 - <csr-id-3d8fa8fef9800b1576beab8a5bc39b821157a5ed/> upgrade edition to 2021 in most crates.
   MSRV for this is 1.56, and we are now at 1.60 so should be compatible.
   This isn't more than a patch release as it should break nobody
   who is adhering to the MSRV, but let's be careful and mark it
   breaking.
   
   Note that `git-features` and `git-pack` are still on edition 2018
   as they make use of a workaround to support (safe) mutable access
   to non-overlapping entries in a slice which doesn't work anymore
   in edition 2021.
 - <csr-id-266d4379e9132fd7dd21e6c8fccb36e125069d6e/> Make `realpath()` easier to use by introducing `realpath_opt()`.
   That way there is consistency about how many symlinks to follow.

### Changed (BREAKING)

 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`
 - <csr-id-1b0ef1856f542e5dc23310be93c2b39a9d84cb80/> Provide more details when classifying submodule directories.
   That way we can avoid feeding `.git` files to `git-repository::open()`
   and avoid work duplication, which ultimately allows to open submodules
   directories of all kinds.

### Other

 - <csr-id-7057ad2251f76756a5b35b6bbba10ab9b6601226/> test `is_git` on macOS exfat
 - <csr-id-08d0c27b0641526aa8b70ee5aadd3658862bef1b/> allow opening repos on macos exfat
 - <csr-id-6106521581029c5c24b23a47bb91c1921edfa0af/> avoid shortening absolute paths
 - <csr-id-136eb37b00c9e7ba0fd0bc2a75dee2ac1b06516d/> adopt git-for-windows exception rules
 - <csr-id-277c41f1185398e14f92247aada8422f6b08afd1/> add cross_fs option to upwards discovery
 - <csr-id-e63e722791a7795cd99048bed834459595c60abc/> add ceiling_dirs option to upwards discovery

### New Features

 - <csr-id-d1b7ec605f8016c52c088477b6b0c5adf7ea0ab2/> read worktree specific configuration to override the one from the shared repository.
   This is intensively used when space checkouts are created, along with
   Cone mode. Thus it's the basis for properly interpreting sparse checkout
   options which are set on a per-worktree basis.
 - <csr-id-2e015a3874ab4db40449ec4b8ffb126d8e1a22a4/> add `is_submodule_git_dir()`
 - <csr-id-4a3e1cfbc537963b851c66754e7724875b49b1f4/> `is_git()` now detects submodule worktrees correctly.
 - <csr-id-aa6fd9702e16b4a2bedb0dd0d2323d06133d1a7f/> `git_discover::is_git()` can detect submodule dirs correctly enough.
   We currently detect them as possibly bare, which could be improved if we
   allow ourselves to see `.git/modules` as `submodule` always.
 - <csr-id-010350180459aec41132c960ddafc7b81dd9c04d/> add `DOT_GIT_DIR` constant, containing the name ".git".
 - <csr-id-050f795bfb0fe11655cd7e45c10d87c89ba82625/> allow discovery of linked worktree git dirs
   This also works if the work-tree can't be found but it is otherwise
   a valid git dir.

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### Documentation

 - <csr-id-39ed9eda62b7718d5109135e5ad406fb1fe2978c/> fix typos

### Bug Fixes

 - <csr-id-1a489bee7bb5e9cb168fd2758930dc0b92040dcc/> search dirs that are equal to ceiling dirs are allowed to 'break through' said ceiling.
   This makes the discovery behaviour similar to what `git` does, even though
   by default we consider it an error if that happens as no ceiling directory
   matches.
 - <csr-id-c91ac5b0916a2d2e1bb9ca096e50d627f1f6a8f3/> ignore cwd as ceiling directory
 - <csr-id-70169794bc27936382e75e75e18806649630d7cf/> `upwards::Options::apply_environment()` uses a more 'git'-like default setup.
   When environment variables are used, this won't lead to an error just because the ceiling directories
   don't cover the paths that are currently used for discovery, making it more similar to
   the behaviour git exhibits as well.
 - <csr-id-8840b649b68dd2ae17e43a61ab04cf866124ef46/> make `parse_ceiling_dirs` resilient to illformed UTF-8
   Previously it performed a conversion from bytes to OsStr which might have failed,
   now it stays with `OsStr`all the way.
 - <csr-id-1eddfbd22f1d7362f15021897b0a4fecda5f4c3f/> split directories by semicolon on Windows
 - <csr-id-07cd4a869d22476f812e4dafc847894ddebff5a8/> `GIT_CEILING_DIRECTORIES` now handles UNC paths correctly #(730).
   Previously ceiling directories that were UNC paths would not work together
   with directories which were not due to different bases.
   
   Now ceiling directories will be converted non-UNC paths before using them.
 - <csr-id-40f7379b7a89f7fe6f916801384e9e65e5b85c57/> improve error verbosity when fetching and cloning
 - <csr-id-6fc5c06482636540804f7a8fb74794f52c72eda6/> Discover repo with relative path and ceiling
   A couple of problems are repaired to allow discovering a repository from
   "." with a ceiling directory.
   
   One problem was that find_ceiling_height() did the wrong thing when
   confronted with any relative search_dir. This is resolved by converting
   search_dir to be absolute if it is relative.
   
   The other problem was that discover_opts() also mishandled relative paths.
   When the cursor started out as ".", cursor.pop() would be blindly called
   such that cursor would be "" for the second iteration. When a ceiling
   directory was in use such that there was a max_height, the current height
   would be burned going from ".", to "", and then to "<cwd>", before finally
   actually getting to a real parent directory. This problem is ameliorated by
   testing whether the cursor has a non-empty parent before popping.
   
   N.B. the new test case relies on the test running from the git-discover
   directory such that the gitoxide repository will be found. This is a bit
   fragile and will fail if, for example, the test is run from an unpacked
   gitoxide source tarball.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 268 commits contributed to the release over the course of 287 calendar days.
 - 28 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 10 unique issues were worked on: [#301](https://github.com/Byron/gitoxide/issues/301), [#331](https://github.com/Byron/gitoxide/issues/331), [#426](https://github.com/Byron/gitoxide/issues/426), [#427](https://github.com/Byron/gitoxide/issues/427), [#429](https://github.com/Byron/gitoxide/issues/429), [#450](https://github.com/Byron/gitoxide/issues/450), [#470](https://github.com/Byron/gitoxide/issues/470), [#482](https://github.com/Byron/gitoxide/issues/482), [#544](https://github.com/Byron/gitoxide/issues/544), [#691](https://github.com/Byron/gitoxide/issues/691)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 9 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - update changelogs prior to release ([`84cb256`](https://github.com/Byron/gitoxide/commit/84cb25614a5fcddff297c1713eba4efbb6ff1596))
    - a test to assure non-existing worktree directories don't hinder discovery ([`131481c`](https://github.com/Byron/gitoxide/commit/131481cac26959d46f62d40adcfc895faa4a1698))
    - allow discovery of linked worktree git dirs ([`050f795`](https://github.com/Byron/gitoxide/commit/050f795bfb0fe11655cd7e45c10d87c89ba82625))
    - path shortening now calculates the actual path length, not just component count ([`c0d24e0`](https://github.com/Byron/gitoxide/commit/c0d24e0fb422f990213606d781ba056c238bd70b))
    - adjust to changes in git-path ([`a627ddc`](https://github.com/Byron/gitoxide/commit/a627ddc48be034f8a42d86edf2a9295299297d59))
    - adapt to changes in git-path ([`c258d77`](https://github.com/Byron/gitoxide/commit/c258d77660755559eaa5389bfabe0cebdf478a80))
    - refactor ([`81a542f`](https://github.com/Byron/gitoxide/commit/81a542fbb680a19322877cf3740ae2ae66b95011))
    - rely on `absolutize_components()` ([`e844006`](https://github.com/Byron/gitoxide/commit/e84400660dad6281fe3869ad649470f2adf31979))
    - brutally fix path handling of common dirs ([`e120232`](https://github.com/Byron/gitoxide/commit/e120232252875cd3fdacb9b7df90c3db58e7e24e))
    - Another test to assure invalid worktree dirs aren't git dirs ([`e6e3608`](https://github.com/Byron/gitoxide/commit/e6e3608490bda2577c23f93d905a51c33fa4f777))
    - adjust for different errors on windows when handling errors opening files… ([`9625829`](https://github.com/Byron/gitoxide/commit/962582996bb8d53739393acfcd150e9aa5132bae))
    - fix tests on windows ([`8080ad2`](https://github.com/Byron/gitoxide/commit/8080ad2baedf83237a0d9a01550582fc8592e568))
    - better tests for worktree discovery ([`9d0f134`](https://github.com/Byron/gitoxide/commit/9d0f134b3186635cbc253464fc7cec2ff522a4e2))
    - The first working version of worktree detection ([`dc73e96`](https://github.com/Byron/gitoxide/commit/dc73e96fdd21713083d9fe0464d1a5689405c116))
    - fix windows tests ([`b025008`](https://github.com/Byron/gitoxide/commit/b0250080c5a1e8ba3fbdb3f70038521b75634306))
    - refactor ([`6b73e0d`](https://github.com/Byron/gitoxide/commit/6b73e0dc60488fcadf21366322e75f711bada6f4))
    - support for gitdir file parsing with all of gits safety ([`1223fc7`](https://github.com/Byron/gitoxide/commit/1223fc7777f57d73c3693407f6a1e41393919a99))
    - basic parsing for git-dir files ([`e11c677`](https://github.com/Byron/gitoxide/commit/e11c67770c301942188f204dbb2cd61880087959))
    - fix test on windows maybe ([`9583048`](https://github.com/Byron/gitoxide/commit/95830480928421de1e38c99a7e9a65629ec1cc41))
    - Try to fix winodws build ([`f2c5ed3`](https://github.com/Byron/gitoxide/commit/f2c5ed3326a3076cc25afbaeec9a540d823e0606))
    - refactor `repositoryKind` adjusted to handle linked worktrees ([`84677cb`](https://github.com/Byron/gitoxide/commit/84677cb09634e1d18ce20850bb7c6c9d63a13818))
    - refactor ([`eead214`](https://github.com/Byron/gitoxide/commit/eead2144e10b96e997cd702cb651f4c92693df95))
    - avoid running failing test on _any_ windows ([`9351a3d`](https://github.com/Byron/gitoxide/commit/9351a3db2a7e9da13be3071c1d2d5aa0cb7a35b6))
    - Fix tests on windows maybe ([`8084581`](https://github.com/Byron/gitoxide/commit/8084581619c7261f779ae42856be756eac7fbfa4))
    - don't archive repositories with worktrees ([`389ef66`](https://github.com/Byron/gitoxide/commit/389ef663ce0417f3840f90dedfc91a57eae6fd0c))
    - prepare test to provide worktrees of different kinds as well ([`a4cec4a`](https://github.com/Byron/gitoxide/commit/a4cec4ab8ba80a8f439b283d369d48cdafbe7b74))
    - Add required support for reading path files ([`bc08511`](https://github.com/Byron/gitoxide/commit/bc085116241e8824c623efc9893ded220fe9a5a2))
    - Remove handling of environment variables ([`4e914f8`](https://github.com/Byron/gitoxide/commit/4e914f89bbf04dbe55eae01f7be09899296ad8b0))
    - Add git-discover to check-size ([`85b29a6`](https://github.com/Byron/gitoxide/commit/85b29a6ba6b3ee4b925fd10c688b2aabcf4273f2))
    - fix docs ([`1e3acd0`](https://github.com/Byron/gitoxide/commit/1e3acd08b9df9fe0cc36bb6a4d4bac57c365443d))
    - refactor ([`00a988e`](https://github.com/Byron/gitoxide/commit/00a988e3c2c964447f675164a6126bf6cb470c6b))
    - Migrate all relevant code from git-repository to git-discover ([`631e70e`](https://github.com/Byron/gitoxide/commit/631e70e6f210df40eb789023970ec17095ec3556))
    - Add empty git-discover crate ([`6565f16`](https://github.com/Byron/gitoxide/commit/6565f163ba464f5adf63d214143a4e12f48b00fc))
 * **[#331](https://github.com/Byron/gitoxide/issues/331)**
    - add `DOT_GIT_DIR` constant, containing the name ".git". ([`0103501`](https://github.com/Byron/gitoxide/commit/010350180459aec41132c960ddafc7b81dd9c04d))
    - adjustments due to breaking changes in `git_path` ([`4420ae9`](https://github.com/Byron/gitoxide/commit/4420ae932d5b20a9662a6d36353a27111b5cd672))
 * **[#426](https://github.com/Byron/gitoxide/issues/426)**
    - allow tests to deal with shared drives by adjusting trust expectation ([`85ab096`](https://github.com/Byron/gitoxide/commit/85ab0964a1a35556a5efe392576c9b028e601c6c))
    - cleanup ([`e029eb4`](https://github.com/Byron/gitoxide/commit/e029eb4589942e06a53bc0f6ed472d1d33c4a7e6))
 * **[#427](https://github.com/Byron/gitoxide/issues/427)**
    - make fmt ([`4b320e7`](https://github.com/Byron/gitoxide/commit/4b320e773368ac5e8c38dd8a779ef3d6d2d024ec))
    - Turn on performance mode for sha-1 computation ([`44371a1`](https://github.com/Byron/gitoxide/commit/44371a10f464f32db346aa6b8309e983cfa20933))
 * **[#429](https://github.com/Byron/gitoxide/issues/429)**
    - Adjust changelogs prior to release ([`7397805`](https://github.com/Byron/gitoxide/commit/7397805fd032a752d6c2f2c2c28ac11ddecc7193))
 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - upgrade `bstr` to `1.0.1` ([`99905ba`](https://github.com/Byron/gitoxide/commit/99905bacace8aed42b16d43f0f04cae996cb971c))
    - adapt to changes in `git-sec` ([`0ce21b1`](https://github.com/Byron/gitoxide/commit/0ce21b1e95e00f2d4cebc1c62349e0dc66a1c705))
 * **[#470](https://github.com/Byron/gitoxide/issues/470)**
    - update changelogs prior to release ([`caa7a1b`](https://github.com/Byron/gitoxide/commit/caa7a1bdef74d7d3166a7e38127a59f5ab3cfbdd))
 * **[#482](https://github.com/Byron/gitoxide/issues/482)**
    - add `is_submodule_git_dir()` ([`2e015a3`](https://github.com/Byron/gitoxide/commit/2e015a3874ab4db40449ec4b8ffb126d8e1a22a4))
    - Query the `cwd` only once instead of potentially multiple times, allocating a Vec each time. ([`6be38f2`](https://github.com/Byron/gitoxide/commit/6be38f294087f8c5e9bb8522bbf928a761c5cdf0))
    - Provide more details when classifying submodule directories. ([`1b0ef18`](https://github.com/Byron/gitoxide/commit/1b0ef1856f542e5dc23310be93c2b39a9d84cb80))
    - fix windows build ([`ff488fa`](https://github.com/Byron/gitoxide/commit/ff488fac628b95ec8f64b136a125913b7b96d3a6))
    - fix docs ([`3724f31`](https://github.com/Byron/gitoxide/commit/3724f31c0563d79a0cad88d773fa60ea21f504b8))
    - `is_git()` now detects submodule worktrees correctly. ([`4a3e1cf`](https://github.com/Byron/gitoxide/commit/4a3e1cfbc537963b851c66754e7724875b49b1f4))
    - refactor ([`1ee9918`](https://github.com/Byron/gitoxide/commit/1ee991847a5adeaaeb6e80ae626c28b0ba89e0af))
    - `git_discover::is_git()` can detect submodule dirs correctly enough. ([`aa6fd97`](https://github.com/Byron/gitoxide/commit/aa6fd9702e16b4a2bedb0dd0d2323d06133d1a7f))
    - another test indicating git-discover misclassifies `.git/modules` as worktree ([`9133141`](https://github.com/Byron/gitoxide/commit/91331416146afb9e868e0868b47097865d2d7117))
    - Failing test to show discovery skips submodules entirely ([`b56b57c`](https://github.com/Byron/gitoxide/commit/b56b57c4111512853e42397ee1fdfd4c11cadf75))
 * **[#544](https://github.com/Byron/gitoxide/issues/544)**
    - Fix a test that assumed a non-worktree parent repo. ([`1908229`](https://github.com/Byron/gitoxide/commit/190822908d99c7e2f9f118cd1eff80607e4558c4))
 * **[#691](https://github.com/Byron/gitoxide/issues/691)**
    - set `rust-version` to 1.64 ([`55066ce`](https://github.com/Byron/gitoxide/commit/55066ce5fd71209abb5d84da2998b903504584bb))
 * **Uncategorized**
    - Release gix-features v0.26.4, gix-actor v0.17.1, gix-glob v0.5.3, gix-path v0.7.1, gix-quote v0.4.1, gix-attributes v0.8.2, gix-config-value v0.10.1, gix-tempfile v3.0.2, gix-lock v3.0.2, gix-validate v0.7.2, gix-object v0.26.1, gix-ref v0.24.0, gix-sec v0.6.2, gix-config v0.16.1, gix-command v0.2.3, gix-prompt v0.3.2, gix-url v0.13.2, gix-credentials v0.9.1, gix-diff v0.26.1, gix-discover v0.13.0, gix-hashtable v0.1.1, gix-bitmap v0.2.1, gix-traverse v0.22.1, gix-index v0.12.3, gix-mailmap v0.9.2, gix-chunk v0.4.1, gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`6efd0d3`](https://github.com/Byron/gitoxide/commit/6efd0d31fbeca31ab7319aa2ac97bb31dc4ce055))
    - Release gix-date v0.4.2, gix-hash v0.10.2, gix-features v0.26.4, gix-actor v0.17.1, gix-glob v0.5.3, gix-path v0.7.1, gix-quote v0.4.1, gix-attributes v0.8.2, gix-config-value v0.10.1, gix-tempfile v3.0.2, gix-lock v3.0.2, gix-validate v0.7.2, gix-object v0.26.1, gix-ref v0.24.0, gix-sec v0.6.2, gix-config v0.16.1, gix-command v0.2.3, gix-prompt v0.3.2, gix-url v0.13.2, gix-credentials v0.9.1, gix-diff v0.26.1, gix-discover v0.13.0, gix-hashtable v0.1.1, gix-bitmap v0.2.1, gix-traverse v0.22.1, gix-index v0.12.3, gix-mailmap v0.9.2, gix-chunk v0.4.1, gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`6ccc88a`](https://github.com/Byron/gitoxide/commit/6ccc88a8e4a56973b1a358cf72dc012ee3c75d56))
    - Merge branch 'rename-crates' into inform-about-gix-rename ([`c9275b9`](https://github.com/Byron/gitoxide/commit/c9275b99ea43949306d93775d9d78c98fb86cfb1))
    - rename `git-testtools` to `gix-testtools` ([`b65c33d`](https://github.com/Byron/gitoxide/commit/b65c33d256cfed65d11adeff41132e3e58754089))
    - adjust to renaming of `git-pack` to `gix-pack` ([`1ee81ad`](https://github.com/Byron/gitoxide/commit/1ee81ad310285ee4aa118118a2be3810dbace574))
    - adjust to renaming of `git-odb` to `gix-odb` ([`476e2ad`](https://github.com/Byron/gitoxide/commit/476e2ad1a64e9e3f0d7c8651d5bcbee36cd78241))
    - adjust to renaming of `git-index` to `gix-index` ([`86db5e0`](https://github.com/Byron/gitoxide/commit/86db5e09fc58ce66b252dc13b8d7e2c48e4d5062))
    - adjust to renaming of `git-diff` to `gix-diff` ([`49a163e`](https://github.com/Byron/gitoxide/commit/49a163ec8b18f0e5fcd05a315de16d5d8be7650e))
    - adjust to renaming of `git-commitgraph` to `gix-commitgraph` ([`f1dd0a3`](https://github.com/Byron/gitoxide/commit/f1dd0a3366e31259af029da73228e8af2f414244))
    - adjust to renaming of `git-mailmap` to `gix-mailmap` ([`2e28c56`](https://github.com/Byron/gitoxide/commit/2e28c56bb9f70de6f97439818118d3a25859698f))
    - adjust to renaming of `git-discover` to `gix-discover` ([`53adfe1`](https://github.com/Byron/gitoxide/commit/53adfe1c34e9ea3b27067a97b5e7ac80b351c441))
    - rename `git-discover` to `gix-discover` ([`6e619de`](https://github.com/Byron/gitoxide/commit/6e619deb6fd063b7fe524b9e2f837c2168adeaa3))
    - adjust to renaming of `git-chunk` to `gix-chunk` ([`59194e3`](https://github.com/Byron/gitoxide/commit/59194e3a07853eae0624ebc4907478d1de4f7599))
    - adjust to renaming of `git-bitmap` to `gix-bitmap` ([`75f2a07`](https://github.com/Byron/gitoxide/commit/75f2a079b17489f62bc43e1f1d932307375c4f9d))
    - adjust to renaming for `git-protocol` to `gix-protocol` ([`823795a`](https://github.com/Byron/gitoxide/commit/823795addea3810243cab7936cd8ec0137cbc224))
    - adjust to renaming of `git-refspec` to `gix-refspec` ([`c958802`](https://github.com/Byron/gitoxide/commit/c9588020561577736faa065e7e5b5bb486ca8fe1))
    - adjust to renaming of `git-revision` to `gix-revision` ([`ee0ee84`](https://github.com/Byron/gitoxide/commit/ee0ee84607c2ffe11ee75f27a31903db68afed02))
    - adjust to renaming of `git-transport` to `gix-transport` ([`b2ccf71`](https://github.com/Byron/gitoxide/commit/b2ccf716dc4425bb96651d4d58806a3cc2da219e))
    - adjust to renaming of `git-credentials` to `gix-credentials` ([`6b18abc`](https://github.com/Byron/gitoxide/commit/6b18abcf2856f02ab938d535a65e51ac282bf94a))
    - adjust to renaming of `git-prompt` to `gix-prompt` ([`6a4654e`](https://github.com/Byron/gitoxide/commit/6a4654e0d10ab773dd219cb4b731c0fc1471c36d))
    - adjust to renaming of `git-command` to `gix-command` ([`d26b8e0`](https://github.com/Byron/gitoxide/commit/d26b8e046496894ae06b0bbfdba77196976cd975))
    - adjust to renaming of `git-packetline` to `gix-packetline` ([`5cbd22c`](https://github.com/Byron/gitoxide/commit/5cbd22cf42efb760058561c6c3bbcd4dab8c8be1))
    - adjust to renaming of `git-worktree` to `gix-worktree` ([`73a1282`](https://github.com/Byron/gitoxide/commit/73a12821b3d9b66ec1714d07dd27eb7a73e3a544))
    - adjust to renamining of `git-hashtable` to `gix-hashtable` ([`26a0c98`](https://github.com/Byron/gitoxide/commit/26a0c98d0a389b03e3dc7bfc758b37155e285244))
    - adjust to renamining of `git-worktree` to `gix-worktree` ([`108bb1a`](https://github.com/Byron/gitoxide/commit/108bb1a634f4828853fb590e9fc125f79441dd38))
    - adjust to renaming of `git-url` to `gix-url` ([`b50817a`](https://github.com/Byron/gitoxide/commit/b50817aadb143e19f61f64e19b19ec1107d980c6))
    - adjust to renaming of `git-date` to `gix-date` ([`9a79ff2`](https://github.com/Byron/gitoxide/commit/9a79ff2d5cc74c1efad9f41e21095ae498cce00b))
    - adjust to renamining of `git-attributes` to `gix-attributes` ([`4a8b3b8`](https://github.com/Byron/gitoxide/commit/4a8b3b812ac26f2a2aee8ce8ca81591273383c84))
    - adjust to renaminig of `git-quote` to `gix-quote` ([`648025b`](https://github.com/Byron/gitoxide/commit/648025b7ca94411fdd0d90c53e5faede5fde6c8d))
    - adjust to renaming of `git-config` to `gix-config` ([`3a861c8`](https://github.com/Byron/gitoxide/commit/3a861c8f049f6502d3bcbdac752659aa1aeda46a))
    - adjust to renaming of `git-ref` to `gix-ref` ([`1f5f695`](https://github.com/Byron/gitoxide/commit/1f5f695407b034377d94b172465ff573562b3fc3))
    - adjust to renaming of `git-lock` to `gix-lock` ([`2028e78`](https://github.com/Byron/gitoxide/commit/2028e7884ae1821edeec81612f501e88e4722b17))
    - adjust to renaming of `git-tempfile` to `gix-tempfile` ([`b6cc3eb`](https://github.com/Byron/gitoxide/commit/b6cc3ebb5137084a6327af16a7d9364d8f092cc9))
    - adjust to renaming of `git-object` to `gix-object` ([`fc86a1e`](https://github.com/Byron/gitoxide/commit/fc86a1e710ad7bf076c25cc6f028ddcf1a5a4311))
    - adjust to renaming of `git-actor` to `gix-actor` ([`4dc9b44`](https://github.com/Byron/gitoxide/commit/4dc9b44dc52f2486ffa2040585c6897c1bf55df4))
    - adjust to renaming of `git-validate` to `gix-validate` ([`5e40ad0`](https://github.com/Byron/gitoxide/commit/5e40ad078af3d08cbc2ca81ce755c0ed8a065b4f))
    - adjust to renaming of `git-hash` to `gix-hash` ([`4a9d025`](https://github.com/Byron/gitoxide/commit/4a9d0257110c3efa61d08c8457c4545b200226d1))
    - adjust to renaming of `git-features` to `gix-features` ([`e2dd68a`](https://github.com/Byron/gitoxide/commit/e2dd68a417aad229e194ff20dbbfd77668096ec6))
    - adjust to renaming of `git-glob` to `gix-glob` ([`35b2a3a`](https://github.com/Byron/gitoxide/commit/35b2a3acbc8f2a03f151bc0a3863163844e0ca86))
    - adjust to renaming of `git-sec` to `gix-sec` ([`eabbb92`](https://github.com/Byron/gitoxide/commit/eabbb923bd5a32fc80fa80f96cfdc2ab7bb2ed17))
    - adapt to renaming of `git-path` to `gix-path` ([`d3bbcfc`](https://github.com/Byron/gitoxide/commit/d3bbcfccad80fc44ea8e7bf819f23adaca06ba2d))
    - adjust to rename of `git-config-value` to `gix-config-value` ([`622b3e1`](https://github.com/Byron/gitoxide/commit/622b3e1d0bffa0f8db73697960f9712024fac430))
    - Merge branch 'unc-paths' ([`ff0387e`](https://github.com/Byron/gitoxide/commit/ff0387e9975e61a2d796b86f4d857c3b8528c94b))
    - `GIT_CEILING_DIRECTORIES` now handles UNC paths correctly #(730). ([`07cd4a8`](https://github.com/Byron/gitoxide/commit/07cd4a869d22476f812e4dafc847894ddebff5a8))
    - Release git-date v0.4.2, git-hash v0.10.2, git-features v0.26.2, git-actor v0.17.1, git-glob v0.5.3, git-path v0.7.1, git-quote v0.4.1, git-attributes v0.8.2, git-config-value v0.10.1, git-tempfile v3.0.2, git-lock v3.0.2, git-validate v0.7.2, git-object v0.26.1, git-ref v0.24.0, git-sec v0.6.2, git-config v0.16.0, git-command v0.2.3, git-prompt v0.3.2, git-url v0.13.2, git-credentials v0.9.1, git-diff v0.26.1, git-discover v0.13.0, git-hashtable v0.1.1, git-bitmap v0.2.1, git-traverse v0.22.1, git-index v0.12.3, git-mailmap v0.9.2, git-chunk v0.4.1, git-pack v0.30.2, git-odb v0.40.2, git-packetline v0.14.2, git-transport v0.25.4, git-protocol v0.26.3, git-revision v0.10.2, git-refspec v0.7.2, git-worktree v0.12.2, git-repository v0.34.0, safety bump 3 crates ([`c196d20`](https://github.com/Byron/gitoxide/commit/c196d206d57a310b1ce974a1cf0e7e6d6db5c4d6))
    - prepare changelogs prior to release ([`7c846d2`](https://github.com/Byron/gitoxide/commit/7c846d2102dc767366771925212712ef8cc9bf07))
    - Merge branch 'Lioness100/main' ([`1e544e8`](https://github.com/Byron/gitoxide/commit/1e544e82455bf9ecb5e3c2146280eaf7ecd81f16))
    - fix typos ([`39ed9ed`](https://github.com/Byron/gitoxide/commit/39ed9eda62b7718d5109135e5ad406fb1fe2978c))
    - search dirs that are equal to ceiling dirs are allowed to 'break through' said ceiling. ([`1a489be`](https://github.com/Byron/gitoxide/commit/1a489bee7bb5e9cb168fd2758930dc0b92040dcc))
    - Add test to help understand why it acts the way it does. ([`48057d3`](https://github.com/Byron/gitoxide/commit/48057d305b5bddcd6a05a73b80d1c17045b54ade))
    - ignore cwd as ceiling directory ([`c91ac5b`](https://github.com/Byron/gitoxide/commit/c91ac5b0916a2d2e1bb9ca096e50d627f1f6a8f3))
    - upgrade serial-test to 1.0 ([`9ab3e25`](https://github.com/Byron/gitoxide/commit/9ab3e25e4c1ff6d504a79521cfa9453a7e7e7465))
    - thanks clippy ([`bac57dd`](https://github.com/Byron/gitoxide/commit/bac57dd05ea2d5a4ee45ef9350fa3f2e19474bc0))
    - make fmt ([`e22080e`](https://github.com/Byron/gitoxide/commit/e22080e4a29d0bad15a99d565a5e3e304a8743ec))
    - `upwards::Options::apply_environment()` uses a more 'git'-like default setup. ([`7016979`](https://github.com/Byron/gitoxide/commit/70169794bc27936382e75e75e18806649630d7cf))
    - Allow ceiling dirs not to match if `apply_environment` is used ([`9b7df62`](https://github.com/Byron/gitoxide/commit/9b7df6237611cd1b6328364e6d95c22c99c63ca7))
    - Merge branch 'fix/ceiling-directories-win' ([`78517a6`](https://github.com/Byron/gitoxide/commit/78517a616b4b1b6fefc2367057a9b3b786d93750))
    - make `parse_ceiling_dirs` resilient to illformed UTF-8 ([`8840b64`](https://github.com/Byron/gitoxide/commit/8840b649b68dd2ae17e43a61ab04cf866124ef46))
    - split directories by semicolon on Windows ([`1eddfbd`](https://github.com/Byron/gitoxide/commit/1eddfbd22f1d7362f15021897b0a4fecda5f4c3f))
    - Release git-date v0.4.1, git-features v0.26.1, git-glob v0.5.2, git-attributes v0.8.1, git-tempfile v3.0.1, git-ref v0.23.1, git-sec v0.6.1, git-config v0.15.1, git-prompt v0.3.1, git-url v0.13.1, git-discover v0.12.1, git-index v0.12.2, git-mailmap v0.9.1, git-pack v0.30.1, git-odb v0.40.1, git-transport v0.25.3, git-protocol v0.26.2, git-revision v0.10.1, git-refspec v0.7.1, git-worktree v0.12.1, git-repository v0.33.0 ([`5b5b380`](https://github.com/Byron/gitoxide/commit/5b5b3809faa71c658db38b40dfc410224d08a367))
    - prepare changelogs prior to release ([`93bef97`](https://github.com/Byron/gitoxide/commit/93bef97b3c0c75d4bf7119fdd787516e1efc77bf))
    - Merge branch 'patch-1' ([`b93f0c4`](https://github.com/Byron/gitoxide/commit/b93f0c49fc677b6c19aea332cbfc1445ce475375))
    - thanks clippy ([`9e04685`](https://github.com/Byron/gitoxide/commit/9e04685dd3f109bfb27663f9dc7c04102e660bf2))
    - uprgade serial_test to 0.10 ([`8623b5a`](https://github.com/Byron/gitoxide/commit/8623b5a36f3e82cd694f7772798e36656cbfcd0a))
    - Release git-ref v0.23.0, git-config v0.15.0, git-command v0.2.2, git-diff v0.26.0, git-discover v0.12.0, git-mailmap v0.9.0, git-pack v0.30.0, git-odb v0.40.0, git-transport v0.25.2, git-protocol v0.26.1, git-revision v0.10.0, git-refspec v0.7.0, git-worktree v0.12.0, git-repository v0.32.0 ([`ffb5b6a`](https://github.com/Byron/gitoxide/commit/ffb5b6a21cb415315db6fd5294940c7c6deb4538))
    - prepare changelogs prior to release ([`4381a03`](https://github.com/Byron/gitoxide/commit/4381a03a34c305f31713cce234c2afbf8ac60f01))
    - Release git-date v0.4.0, git-actor v0.17.0, git-object v0.26.0, git-traverse v0.22.0, git-index v0.12.0, safety bump 15 crates ([`0e3d0a5`](https://github.com/Byron/gitoxide/commit/0e3d0a56d7e6a60c6578138f2690b4fa54a2072d))
    - Release git-features v0.26.0, git-actor v0.16.0, git-attributes v0.8.0, git-object v0.25.0, git-ref v0.22.0, git-config v0.14.0, git-command v0.2.1, git-url v0.13.0, git-credentials v0.9.0, git-diff v0.25.0, git-discover v0.11.0, git-traverse v0.21.0, git-index v0.11.0, git-mailmap v0.8.0, git-pack v0.29.0, git-odb v0.39.0, git-transport v0.25.0, git-protocol v0.26.0, git-revision v0.9.0, git-refspec v0.6.0, git-worktree v0.11.0, git-repository v0.31.0, safety bump 24 crates ([`5ac9fbe`](https://github.com/Byron/gitoxide/commit/5ac9fbe265a5b61c533a2a6b3abfed2bdf7f89ad))
    - prepare changelogs prior to release ([`30d8ca1`](https://github.com/Byron/gitoxide/commit/30d8ca19284049dcfbb0de2698cafae1d1a16b0c))
    - Release git-date v0.3.1, git-features v0.25.0, git-actor v0.15.0, git-glob v0.5.1, git-path v0.7.0, git-attributes v0.7.0, git-config-value v0.10.0, git-lock v3.0.1, git-validate v0.7.1, git-object v0.24.0, git-ref v0.21.0, git-sec v0.6.0, git-config v0.13.0, git-prompt v0.3.0, git-url v0.12.0, git-credentials v0.8.0, git-diff v0.24.0, git-discover v0.10.0, git-traverse v0.20.0, git-index v0.10.0, git-mailmap v0.7.0, git-pack v0.28.0, git-odb v0.38.0, git-packetline v0.14.1, git-transport v0.24.0, git-protocol v0.25.0, git-revision v0.8.0, git-refspec v0.5.0, git-worktree v0.10.0, git-repository v0.30.0, safety bump 26 crates ([`e6b9906`](https://github.com/Byron/gitoxide/commit/e6b9906c486b11057936da16ed6e0ec450a0fb83))
    - prepare chnagelogs prior to git-repository release ([`7114bbb`](https://github.com/Byron/gitoxide/commit/7114bbb6732aa8571d4ab74f28ed3e26e9fbe4d0))
    - Merge branch 'main' into read-split-index ([`c57bdde`](https://github.com/Byron/gitoxide/commit/c57bdde6de37eca9672ea715962bbd02aa3eb055))
    - Merge branch 'adjustments-for-cargo' ([`083909b`](https://github.com/Byron/gitoxide/commit/083909bc7eb902eeee2002034fdb6ed88280dc5c))
    - thanks clippy ([`f1160fb`](https://github.com/Byron/gitoxide/commit/f1160fb42acf59b37cbeda546a7079af3c9bc050))
    - adjust to changes in `git-testtools` ([`4eb842c`](https://github.com/Byron/gitoxide/commit/4eb842c7150b980e1c2637217e1f9657a671cea7))
    - Merge branch 'adjustments-for-cargo' ([`94750e1`](https://github.com/Byron/gitoxide/commit/94750e15831969059551af35d31c21009462084d))
    - assure git-discover can recognize a repository without a 'config' file ([`7994416`](https://github.com/Byron/gitoxide/commit/79944160f3cad4b977521853e881070f23b7dcc1))
    - Merge branch 'adjustments-for-cargo' ([`70ccbb2`](https://github.com/Byron/gitoxide/commit/70ccbb21b1113bdeb20b52d274141a9fdb75f579))
    - improve error verbosity when fetching and cloning ([`40f7379`](https://github.com/Byron/gitoxide/commit/40f7379b7a89f7fe6f916801384e9e65e5b85c57))
    - read worktree specific configuration to override the one from the shared repository. ([`d1b7ec6`](https://github.com/Byron/gitoxide/commit/d1b7ec605f8016c52c088477b6b0c5adf7ea0ab2))
    - fix typo in docs ([`2d9b2f1`](https://github.com/Byron/gitoxide/commit/2d9b2f16d938e1a4a391534df37f99f1c64c8ea6))
    - Release git-hash v0.10.1, git-hashtable v0.1.0 ([`7717170`](https://github.com/Byron/gitoxide/commit/771717095d9a67b0625021eb0928828ab686e772))
    - Merge branch 'main' into http-config ([`6b9632e`](https://github.com/Byron/gitoxide/commit/6b9632e16c416841ffff1b767ee7a6c89b421220))
    - Merge branch 'path-normalize' ([`805329a`](https://github.com/Byron/gitoxide/commit/805329a0a5f6543bbc1d5885977b47bf7baa7f08))
    - adjust for changes in `git-path` ([`cf25e35`](https://github.com/Byron/gitoxide/commit/cf25e3594b99909defb431f34fb3a4d8a25bd37c))
    - Merge branch 'main' into http-config ([`bcd9654`](https://github.com/Byron/gitoxide/commit/bcd9654e56169799eb706646da6ee1f4ef2021a9))
    - make fmt ([`0abab7d`](https://github.com/Byron/gitoxide/commit/0abab7da2ec1b8560e6c1eb009f534c9fc7814fe))
    - Release git-hash v0.10.0, git-features v0.24.0, git-date v0.3.0, git-actor v0.14.0, git-glob v0.5.0, git-path v0.6.0, git-quote v0.4.0, git-attributes v0.6.0, git-config-value v0.9.0, git-tempfile v3.0.0, git-lock v3.0.0, git-validate v0.7.0, git-object v0.23.0, git-ref v0.20.0, git-sec v0.5.0, git-config v0.12.0, git-command v0.2.0, git-prompt v0.2.0, git-url v0.11.0, git-credentials v0.7.0, git-diff v0.23.0, git-discover v0.9.0, git-bitmap v0.2.0, git-traverse v0.19.0, git-index v0.9.0, git-mailmap v0.6.0, git-chunk v0.4.0, git-pack v0.27.0, git-odb v0.37.0, git-packetline v0.14.0, git-transport v0.23.0, git-protocol v0.24.0, git-revision v0.7.0, git-refspec v0.4.0, git-worktree v0.9.0, git-repository v0.29.0, git-commitgraph v0.11.0, gitoxide-core v0.21.0, gitoxide v0.19.0, safety bump 28 crates ([`b2c301e`](https://github.com/Byron/gitoxide/commit/b2c301ef131ffe1871314e19f387cf10a8d2ac16))
    - prepare changelogs prior to release ([`e4648f8`](https://github.com/Byron/gitoxide/commit/e4648f827c97e9d13636d1bbdc83dd63436e6e5c))
    - Merge branch 'discover-rel-path' ([`5f908fb`](https://github.com/Byron/gitoxide/commit/5f908fb86857d565715b9b0b8b453b29273fb022))
    - add test that actually uses chdir like in the real world. ([`048c8b2`](https://github.com/Byron/gitoxide/commit/048c8b260f78ee1eac74bf8f4ee2374d6b0eb308))
    - refactor ([`cdb9556`](https://github.com/Byron/gitoxide/commit/cdb95567c83d52cf93a621bf616883f09f5544bb))
    - Discover repo with relative path and ceiling ([`6fc5c06`](https://github.com/Byron/gitoxide/commit/6fc5c06482636540804f7a8fb74794f52c72eda6))
    - Merge branch 'cwd-consistency' ([`ea7c6a3`](https://github.com/Byron/gitoxide/commit/ea7c6a3b069c9e13905b51b87538c57ba9182dca))
    - `Path::from_dot_git_dir()` now takes the `current_dir` as argument and returns `Option<path>` ([`b4dcfc7`](https://github.com/Byron/gitoxide/commit/b4dcfc716a80ffccbab6f7ccc586d8063cc10fff))
    - Merge branch 'version2021' ([`0e4462d`](https://github.com/Byron/gitoxide/commit/0e4462df7a5166fe85c23a779462cdca8ee013e8))
    - upgrade edition to 2021 in most crates. ([`3d8fa8f`](https://github.com/Byron/gitoxide/commit/3d8fa8fef9800b1576beab8a5bc39b821157a5ed))
    - Release git-glob v0.4.2, git-config-value v0.8.2, git-lock v2.2.0, git-ref v0.19.0, git-config v0.11.0, git-discover v0.8.0, git-index v0.8.0, git-transport v0.22.0, git-protocol v0.23.0, git-worktree v0.8.0, git-repository v0.28.0, gitoxide-core v0.20.0, gitoxide v0.18.0, safety bump 9 crates ([`0c253b1`](https://github.com/Byron/gitoxide/commit/0c253b15143dcedfe4c66d64ab1ea6e097030651))
    - prepare changelogs prior to release ([`fe5721f`](https://github.com/Byron/gitoxide/commit/fe5721f888c64c79fe9a734a9e33b94a282f8d97))
    - Release git-features v0.23.1, git-glob v0.4.1, git-config-value v0.8.1, git-tempfile v2.0.6, git-object v0.22.1, git-ref v0.18.0, git-sec v0.4.2, git-config v0.10.0, git-prompt v0.1.1, git-url v0.10.1, git-credentials v0.6.1, git-diff v0.21.0, git-discover v0.7.0, git-index v0.7.0, git-pack v0.25.0, git-odb v0.35.0, git-transport v0.21.1, git-protocol v0.22.0, git-refspec v0.3.1, git-worktree v0.7.0, git-repository v0.26.0, git-commitgraph v0.10.0, gitoxide-core v0.19.0, gitoxide v0.17.0, safety bump 9 crates ([`d071583`](https://github.com/Byron/gitoxide/commit/d071583c5576fdf5f7717765ffed5681792aa81f))
    - prepare changelogs prior to release ([`423af90`](https://github.com/Byron/gitoxide/commit/423af90c8202d62dc1ea4a76a0df6421d1f0aa06))
    - Merge branch 'main' into write-sparse-index (upgrade to Rust 1.65) ([`5406630`](https://github.com/Byron/gitoxide/commit/5406630466145990b5adbdadb59151036993060d))
    - thanks clippy ([`04cfa63`](https://github.com/Byron/gitoxide/commit/04cfa635a65ae34ad6d22391f2febd2ca7eabca9))
    - Release git-hash v0.9.11, git-features v0.23.0, git-actor v0.13.0, git-attributes v0.5.0, git-object v0.22.0, git-ref v0.17.0, git-sec v0.4.1, git-config v0.9.0, git-url v0.10.0, git-credentials v0.6.0, git-diff v0.20.0, git-discover v0.6.0, git-traverse v0.18.0, git-index v0.6.0, git-mailmap v0.5.0, git-pack v0.24.0, git-odb v0.34.0, git-packetline v0.13.1, git-transport v0.21.0, git-protocol v0.21.0, git-revision v0.6.0, git-refspec v0.3.0, git-worktree v0.6.0, git-repository v0.25.0, safety bump 24 crates ([`104d922`](https://github.com/Byron/gitoxide/commit/104d922add61ab21c534c24ce8ed37cddf3e275a))
    - prepare changelogs for release ([`d232567`](https://github.com/Byron/gitoxide/commit/d23256701a95284857dc8d1cb37c7c94cada973c))
    - Merge branch 'main' into new-http-impl ([`702a161`](https://github.com/Byron/gitoxide/commit/702a161ef11fc959611bf44b70e9ffe04561c7ad))
    - Merge branch 'fix-544' ([`a0dd96b`](https://github.com/Byron/gitoxide/commit/a0dd96b509f46e261482eb4027fbbd56380e3afc))
    - Merge branch 'main' into fetch-pack ([`93917cb`](https://github.com/Byron/gitoxide/commit/93917cb6ecbb30daf3d20bb5a7c65e12211f084f))
    - Don't run typically failing MacOS specific test on CI anymore ([`6c9d18b`](https://github.com/Byron/gitoxide/commit/6c9d18b1435f28c5ef70371d909cc884eeb6b86a))
    - Merge branch 'diff' ([`25a7726`](https://github.com/Byron/gitoxide/commit/25a7726377fbe400ea3c4927d04e9dec99802b7b))
    - Release git-hash v0.9.10, git-features v0.22.5, git-date v0.2.0, git-actor v0.12.0, git-glob v0.4.0, git-path v0.5.0, git-quote v0.3.0, git-attributes v0.4.0, git-config-value v0.8.0, git-tempfile v2.0.5, git-validate v0.6.0, git-object v0.21.0, git-ref v0.16.0, git-sec v0.4.0, git-config v0.8.0, git-discover v0.5.0, git-traverse v0.17.0, git-index v0.5.0, git-worktree v0.5.0, git-testtools v0.9.0, git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0, safety bump 28 crates ([`29a043b`](https://github.com/Byron/gitoxide/commit/29a043be6808a3e9199a9b26bd076fe843afe4f4))
    - Merge branch 'filter-refs' ([`fd14489`](https://github.com/Byron/gitoxide/commit/fd14489f729172d615d0fa1e8dbd605e9eacf69d))
    - Merge branch 'filter-refs' ([`e10554d`](https://github.com/Byron/gitoxide/commit/e10554d2a3b9c027353a432b0c84f7d3797b7cae))
    - make fmt ([`535e967`](https://github.com/Byron/gitoxide/commit/535e967666c6da657ff1b7eff7c64ab27cafb182))
    - Merge branch 'filter-refs-by-spec' ([`5c05198`](https://github.com/Byron/gitoxide/commit/5c051986bd89590a9287d85d84c713d83dfab83a))
    - Merge branch 'main' into filter-refs-by-spec ([`1f6e5ab`](https://github.com/Byron/gitoxide/commit/1f6e5ab15f5fd8d23719b13e6aea59cd231ac0fe))
    - Merge branch 'fix-522' ([`5869e9f`](https://github.com/Byron/gitoxide/commit/5869e9ff2508d5a93c07635277af8764fcb57713))
    - Release git-hash v0.9.9 ([`da0716f`](https://github.com/Byron/gitoxide/commit/da0716f8c27b4f29cfff0e5ce7fcb3d7240f4aeb))
    - Merge branch 'main' into index-from-tree ([`bc64b96`](https://github.com/Byron/gitoxide/commit/bc64b96a2ec781c72d1d4daad38aa7fb8b74f99b))
    - Merge branch 'main' into filter-refs-by-spec ([`51dc828`](https://github.com/Byron/gitoxide/commit/51dc8282fb77b519ff7d2c94c6bd73af306cfe8b))
    - Release git-diff v0.18.1, git-discover v0.4.2, git-traverse v0.16.4, git-repository v0.23.1 ([`2571831`](https://github.com/Byron/gitoxide/commit/2571831e5939bf4ea6f19537b0c1ccd71dc99088))
    - prepare changelog  prior to release ([`fc6b958`](https://github.com/Byron/gitoxide/commit/fc6b9583d0534f70e0c8afdcad46e09a5001d62b))
    - Merge branch 'macos-exfat' ([`f256f8f`](https://github.com/Byron/gitoxide/commit/f256f8fb7603f83d44acda07386f277c65ac652c))
    - refactor ([`2cd1c00`](https://github.com/Byron/gitoxide/commit/2cd1c007330749f69fc1f154ba1a6e7a0f6e23d0))
    - test `is_git` on macOS exfat ([`7057ad2`](https://github.com/Byron/gitoxide/commit/7057ad2251f76756a5b35b6bbba10ab9b6601226))
    - allow opening repos on macos exfat ([`08d0c27`](https://github.com/Byron/gitoxide/commit/08d0c27b0641526aa8b70ee5aadd3658862bef1b))
    - Release git-path v0.4.2, git-config-value v0.7.0 ([`c48fb31`](https://github.com/Byron/gitoxide/commit/c48fb3107d29f9a06868b0c6de40567063a656d1))
    - Merge branch 'main' into filter-refs-by-spec ([`cfa1440`](https://github.com/Byron/gitoxide/commit/cfa144031dbcac2707ab0cec012bc35e78f9c475))
    - Release git-date v0.0.5, git-hash v0.9.8, git-features v0.22.2, git-actor v0.11.3, git-glob v0.3.2, git-quote v0.2.1, git-attributes v0.3.2, git-tempfile v2.0.4, git-lock v2.1.1, git-validate v0.5.5, git-object v0.20.2, git-ref v0.15.2, git-sec v0.3.1, git-config v0.7.0, git-credentials v0.4.0, git-diff v0.17.2, git-discover v0.4.1, git-bitmap v0.1.2, git-index v0.4.2, git-mailmap v0.3.2, git-chunk v0.3.1, git-traverse v0.16.2, git-pack v0.21.2, git-odb v0.31.2, git-packetline v0.12.7, git-url v0.7.2, git-transport v0.19.2, git-protocol v0.19.0, git-revision v0.4.2, git-refspec v0.1.0, git-worktree v0.4.2, git-repository v0.22.0, safety bump 4 crates ([`4974eca`](https://github.com/Byron/gitoxide/commit/4974eca96d525d1ee4f8cad79bb713af7a18bf9d))
    - Release git-path v0.4.1 ([`5e82346`](https://github.com/Byron/gitoxide/commit/5e823462b3deb904f5d6154a7bf114cef1988224))
    - Merge branch 'main' into remote-ls-refs ([`e2ee3de`](https://github.com/Byron/gitoxide/commit/e2ee3ded97e5c449933712883535b30d151c7c78))
    - Merge branch 'docsrs-show-features' ([`31c2351`](https://github.com/Byron/gitoxide/commit/31c235140cad212d16a56195763fbddd971d87ce))
    - uniformize deny attributes ([`f7f136d`](https://github.com/Byron/gitoxide/commit/f7f136dbe4f86e7dee1d54835c420ec07c96cd78))
    - remove default link to cargo doc everywhere ([`533e887`](https://github.com/Byron/gitoxide/commit/533e887e80c5f7ede8392884562e1c5ba56fb9a8))
    - Merge branch 'main' into remote-ls-refs ([`bd5f3e8`](https://github.com/Byron/gitoxide/commit/bd5f3e8db7e0bb4abfb7b0f79f585ab82c3a14ab))
    - Release git-date v0.0.3, git-actor v0.11.1, git-attributes v0.3.1, git-tempfile v2.0.3, git-object v0.20.1, git-ref v0.15.1, git-config v0.6.1, git-diff v0.17.1, git-discover v0.4.0, git-bitmap v0.1.1, git-index v0.4.1, git-mailmap v0.3.1, git-traverse v0.16.1, git-pack v0.21.1, git-odb v0.31.1, git-packetline v0.12.6, git-url v0.7.1, git-transport v0.19.1, git-protocol v0.18.1, git-revision v0.4.0, git-worktree v0.4.1, git-repository v0.21.0, safety bump 5 crates ([`c96473d`](https://github.com/Byron/gitoxide/commit/c96473dce21c3464aacbc0a62d520c1a33172611))
    - prepare changelogs prior to reelase ([`c06ae1c`](https://github.com/Byron/gitoxide/commit/c06ae1c606b6af9c2a12021103d99c2810750d60))
    - Merge branch 'submodule-open' ([`8f5f3ab`](https://github.com/Byron/gitoxide/commit/8f5f3ab588cf0165d50a82365119ad5804745017))
    - thanks clippy ([`2135fb8`](https://github.com/Byron/gitoxide/commit/2135fb80a126afc6d95b9eaa9f1cd32acb46615b))
    - Release git-hash v0.9.7, git-features v0.22.1 ([`232784a`](https://github.com/Byron/gitoxide/commit/232784a59ded3e8016e4257c7e146ad385cdd64a))
    - Merge branch 'rev-parse-delegate' ([`2f506c7`](https://github.com/Byron/gitoxide/commit/2f506c7c2988477b0f97d272a9ac9ed47b236457))
    - Merge pull request #2 from SidneyDouw/main ([`ce885ad`](https://github.com/Byron/gitoxide/commit/ce885ad4c3324c09c83751c32e014f246c748766))
    - Merge branch 'Byron:main' into main ([`9b9ea02`](https://github.com/Byron/gitoxide/commit/9b9ea0275f8ff5862f24cf5a4ca53bb1cd610709))
    - Merge branch 'main' into rev-parse-delegate ([`6da8250`](https://github.com/Byron/gitoxide/commit/6da82507588d3bc849217c11d9a1d398b67f2ed6))
    - Merge branch 'main' into pathspec ([`7b61506`](https://github.com/Byron/gitoxide/commit/7b615060712565f515515e35a3e8346278ad770c))
    - make fmt ([`47724c0`](https://github.com/Byron/gitoxide/commit/47724c0edb382c036a3fc99884becfd2b0740d4b))
    - Release git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0 ([`aa639d8`](https://github.com/Byron/gitoxide/commit/aa639d8c43f3098cc4a5b50614c5ae94a8156928))
    - Release git-hash v0.9.6, git-features v0.22.0, git-date v0.0.2, git-actor v0.11.0, git-glob v0.3.1, git-path v0.4.0, git-attributes v0.3.0, git-tempfile v2.0.2, git-object v0.20.0, git-ref v0.15.0, git-sec v0.3.0, git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0, safety bump 22 crates ([`4737b1e`](https://github.com/Byron/gitoxide/commit/4737b1eea1d4c9a8d5a69fb63ecac5aa5d378ae5))
    - prepare changelog prior to release ([`3c50625`](https://github.com/Byron/gitoxide/commit/3c50625fa51350ec885b0f38ec9e92f9444df0f9))
    - Merge pull request #1 from Byron/main ([`085e76b`](https://github.com/Byron/gitoxide/commit/085e76b121291ed9bd324139105d2bd4117bedf8))
    - Merge branch 'config-comfort' ([`84b98d9`](https://github.com/Byron/gitoxide/commit/84b98d94177ceaf931aaa521e44eca0fa484d2d3))
    - Merge branch 'main' into pathspec ([`89ea12b`](https://github.com/Byron/gitoxide/commit/89ea12b558bcc056b892193ee8fb44b8664b5da4))
    - Merge branch 'main' into cont_include_if ([`0e9df36`](https://github.com/Byron/gitoxide/commit/0e9df364c4cddf006b1de18b8d167319b7cc1186))
    - Remove another special case on windows due to canonicalize() ([`61abb0b`](https://github.com/Byron/gitoxide/commit/61abb0b006292d2122784b032e198cc716fb7b92))
    - Use git_path::realpath in all places that allow it right now ([`229dc91`](https://github.com/Byron/gitoxide/commit/229dc917fc7d9241b85e5818260a6fbdd3a5daaa))
    - Merge branch 'main' into cont_include_if ([`41ea8ba`](https://github.com/Byron/gitoxide/commit/41ea8ba78e74f5c988148367386a1f4f304cb951))
    - Release git-path v0.3.0, safety bump 14 crates ([`400c9be`](https://github.com/Byron/gitoxide/commit/400c9bec49e4ec5351dc9357b246e7677a63ea35))
    - Release git-date v0.0.1, git-hash v0.9.5, git-features v0.21.1, git-actor v0.10.1, git-path v0.2.0, git-attributes v0.2.0, git-ref v0.14.0, git-sec v0.2.0, git-config v0.5.0, git-credentials v0.2.0, git-discover v0.2.0, git-pack v0.20.0, git-odb v0.30.0, git-url v0.6.0, git-transport v0.18.0, git-protocol v0.17.0, git-revision v0.2.1, git-worktree v0.3.0, git-repository v0.19.0, safety bump 13 crates ([`a417177`](https://github.com/Byron/gitoxide/commit/a41717712578f590f04a33d27adaa63171f25267))
    - update changelogs prior to release ([`bb424f5`](https://github.com/Byron/gitoxide/commit/bb424f51068b8a8e762696890a55ab48900ab980))
    - make fmt ([`c665aef`](https://github.com/Byron/gitoxide/commit/c665aef4270c5ee54da89ee015cc0affd6337608))
    - Merge branch 'main' into svetli-n-cont_include_if ([`315c87e`](https://github.com/Byron/gitoxide/commit/315c87e18c6cac0fafa7b4e59fdd3c076a58a45a))
    - refactor ([`ec37cb8`](https://github.com/Byron/gitoxide/commit/ec37cb8005fa272aed2e23e65adc291875b1fd68))
    - refactor ([`b27a8c2`](https://github.com/Byron/gitoxide/commit/b27a8c243cdc14730478c2a94cafdc8ccf5c60d3))
    - refactor ([`06e96a4`](https://github.com/Byron/gitoxide/commit/06e96a435d820a1ef1e567bf93e7b9ca5fa74829))
    - refactor ([`b555bda`](https://github.com/Byron/gitoxide/commit/b555bdae9964628b6d43e00dd7d7ee8fe674d309))
    - Unify the way `dir_made_absolute` is computed ([`48417fc`](https://github.com/Byron/gitoxide/commit/48417fc8d8d3ec0005ef7b6ef35ced09d92282c2))
    - Merge branch 'main' into davidkna-envopen ([`bc0abc6`](https://github.com/Byron/gitoxide/commit/bc0abc643d3329f885f250b6880560dec861150f))
    - refactor ([`7b307f5`](https://github.com/Byron/gitoxide/commit/7b307f5acb7214f4ff674e00167933cdcccc353a))
    - Make `realpath()` easier to use by introducing `realpath_opt()`. ([`266d437`](https://github.com/Byron/gitoxide/commit/266d4379e9132fd7dd21e6c8fccb36e125069d6e))
    - avoid shortening absolute paths ([`6106521`](https://github.com/Byron/gitoxide/commit/6106521581029c5c24b23a47bb91c1921edfa0af))
    - Add discovery opt env-overrides & env discovery helpers ([`e521d39`](https://github.com/Byron/gitoxide/commit/e521d39e1b0f4849280bae1527bf28977eec5093))
    - Release git-sec v0.1.2, git-discover v0.1.3, cargo-smart-release v0.10.2 ([`6cd365e`](https://github.com/Byron/gitoxide/commit/6cd365e2cf6851f5cdecc22f3b1667440ad011b0))
    - Merge branch 'main' into SidneyDouw-pathspec ([`a22b1d8`](https://github.com/Byron/gitoxide/commit/a22b1d88a21311d44509018729c3ef1936cf052a))
    - Merge branch 'davidkna-admin-sec' ([`3d0e2c2`](https://github.com/Byron/gitoxide/commit/3d0e2c2d4ebdbe3dff01846aac3375128353a2e1))
    - Release git-path v0.1.3, git-discover v0.1.2, git-repository v0.18.1, cargo-smart-release v0.10.1 ([`b7399cc`](https://github.com/Byron/gitoxide/commit/b7399cc44ee419355a649a7b0ba7b352cd48b400))
    - prepare for smart-release release ([`2f74cb0`](https://github.com/Byron/gitoxide/commit/2f74cb05e9b2399355af07517fe3c14e4e8724c5))
    - adopt git-for-windows exception rules ([`136eb37`](https://github.com/Byron/gitoxide/commit/136eb37b00c9e7ba0fd0bc2a75dee2ac1b06516d))
    - Merge branch 'davidkna-discover-x-fs' ([`9abaeda`](https://github.com/Byron/gitoxide/commit/9abaeda2d22e2dbb1db1632c6eb637f1458d06e1))
    - refactor ([`aac5169`](https://github.com/Byron/gitoxide/commit/aac5169aa7c753eb3510e61bc01b47a4b7b01c6f))
    - use `defer` to make dmg unmounting more reliable ([`dbc5caa`](https://github.com/Byron/gitoxide/commit/dbc5caa99b050ac7a96cd5e7a73786072c695530))
    - refactor ([`886e26e`](https://github.com/Byron/gitoxide/commit/886e26ee686776ad4caf1864f24cb358432d5c49))
    - Assure cross-fs check doesn't break if cursor is currently empty ([`4546889`](https://github.com/Byron/gitoxide/commit/4546889adb87c0d9d1c5d22785240033dc3b0dc2))
    - Release git-path v0.1.2, git-sec v0.1.1, git-config v0.4.0, git-discover v0.1.1, git-pack v0.19.1, git-repository v0.18.0, cargo-smart-release v0.10.0, safety bump 2 crates ([`ceb6dff`](https://github.com/Byron/gitoxide/commit/ceb6dff13362a2b4318a551893217c1d11643b9f))
    - add cross_fs option to upwards discovery ([`277c41f`](https://github.com/Byron/gitoxide/commit/277c41f1185398e14f92247aada8422f6b08afd1))
    - Re-enable discovery test on windows thanks to use of `realpath()` ([`1f4ae9e`](https://github.com/Byron/gitoxide/commit/1f4ae9e74843d108b261cbfe10e158f74286088a))
    - Fix windows test failure due to //? by ignoring it there. ([`c5fd322`](https://github.com/Byron/gitoxide/commit/c5fd3223db73ee61844477aff0e8a2438d9b2e39))
    - fix windows tests (broke thanks to \\?\), maybe ([`e458b59`](https://github.com/Byron/gitoxide/commit/e458b5946ebf01d9f901769b8547df85ef14afcb))
    - Assure ceiling dirs are comparable after absolutize ([`32a157b`](https://github.com/Byron/gitoxide/commit/32a157b513191c60795924765825dc7dfb0b38c1))
    - Merge branch 'davidkna-discover-ceiling' ([`66944ba`](https://github.com/Byron/gitoxide/commit/66944ba986114ece2d3b31440c721d0e84b4f267))
    - thanks clippy ([`4979d20`](https://github.com/Byron/gitoxide/commit/4979d2071cbd1a98f3d81aacd60dd99f07d3f746))
    - refactor ([`895b772`](https://github.com/Byron/gitoxide/commit/895b772b5855818ad2227cac8dda0be00f9d5189))
    - Control if at least one ceiling dir has to match using an option ([`ca1f3eb`](https://github.com/Byron/gitoxide/commit/ca1f3ebb1306075767597c75bb288a3a0b4ebb41))
    - Revert "remove implicit canonicalization and improve tests" ([`821f3f3`](https://github.com/Byron/gitoxide/commit/821f3f383a21ddfd274cb6bdcd2228717cd21942))
    - refactor ([`6bbc53b`](https://github.com/Byron/gitoxide/commit/6bbc53be867e5292a98c422526185c6a4736a6a5))
    - Merge branch 'main' into git_includeif ([`229d938`](https://github.com/Byron/gitoxide/commit/229d9383bef8844111d2bf3c406a2ea570109c8b))
    - fix `special_relative_base` test ([`ae226ba`](https://github.com/Byron/gitoxide/commit/ae226ba9c08b04621ce3b42e6972e102d8af2b9c))
    - remove implicit canonicalization and improve tests ([`cdbb4c9`](https://github.com/Byron/gitoxide/commit/cdbb4c9db56ef6eb258a9a76691614d4b45c8d7a))
    - refactor ([`429446c`](https://github.com/Byron/gitoxide/commit/429446c87473dca98bf802fa3de020cb58625f63))
    - refactor ([`9673aae`](https://github.com/Byron/gitoxide/commit/9673aae3da57bc12d45389de3034ab0c2c1c27ab))
    - some more assertions and ceiling dirs ([`a30bcb8`](https://github.com/Byron/gitoxide/commit/a30bcb82413103c69827151f25be0b14b1f04f37))
    - add ceiling_dirs option to upwards discovery ([`e63e722`](https://github.com/Byron/gitoxide/commit/e63e722791a7795cd99048bed834459595c60abc))
    - declare `git-discover` usable as it's fully documented ([`e439015`](https://github.com/Byron/gitoxide/commit/e439015288dad806494e0a2ed8d44fb2247de372))
    - Merge branch 'main' into git_includeif ([`598c853`](https://github.com/Byron/gitoxide/commit/598c853087fcf8f77299aa5b9803bcec705c0cd0))
    - Release git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0 ([`349c590`](https://github.com/Byron/gitoxide/commit/349c5904b0dac350838a896759d51576b66880a7))
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
    - make fmt ([`e043807`](https://github.com/Byron/gitoxide/commit/e043807abf364ca46d00760e2f281528efe20c75))
    - Merge branch 'refs-and-worktrees' ([`8131227`](https://github.com/Byron/gitoxide/commit/8131227ddff6f36919b6a0f7b33792ebde0f8ae9))
    - Merge branch 'main' into git_includeif ([`b1bfc8f`](https://github.com/Byron/gitoxide/commit/b1bfc8fe8efb6d8941f54dddd0fcad99aa13ed6c))
    - Merge branch 'basic-worktree-support' ([`e058bda`](https://github.com/Byron/gitoxide/commit/e058bdabf8449b6a6fdff851e3929137d9b71568))
    - thanks clippy ([`7617da0`](https://github.com/Byron/gitoxide/commit/7617da002ef5906f858c0cc2c349f442bafd6239))
    - thanks clippy ([`a084951`](https://github.com/Byron/gitoxide/commit/a084951c72818d7cb2061053078793213890c899))
    - thanks clippy ([`3c9da80`](https://github.com/Byron/gitoxide/commit/3c9da80a39bc6638a38868f71d80e2a8ad337a4a))
    - Release git-discover v0.0.0 ([`2b5cf2b`](https://github.com/Byron/gitoxide/commit/2b5cf2b95a723d885584ac967cf956c28ea35f47))
</details>

## 0.12.1 (2023-01-10)

A maintenance release without user-facing changes.

## 0.12.0 (2023-01-09)

A maintenance release without user-facing changes.

## 0.11.0 (2022-12-30)

A maintenance release without user-facing changes.

## 0.10.0 (2022-12-19)

### New Features

 - <csr-id-d1b7ec605f8016c52c088477b6b0c5adf7ea0ab2/> read worktree specific configuration to override the one from the shared repository.
   This is intensively used when space checkouts are created, along with
   Cone mode. Thus it's the basis for properly interpreting sparse checkout
   options which are set on a per-worktree basis.

### Bug Fixes

 - <csr-id-40f7379b7a89f7fe6f916801384e9e65e5b85c57/> improve error verbosity when fetching and cloning

## 0.9.0 (2022-11-21)

### Bug Fixes

 - <csr-id-6fc5c06482636540804f7a8fb74794f52c72eda6/> Discover repo with relative path and ceiling
   A couple of problems are repaired to allow discovering a repository from
   "." with a ceiling directory.
   
   One problem was that find_ceiling_height() did the wrong thing when
   confronted with any relative search_dir. This is resolved by converting
   search_dir to be absolute if it is relative.
   
   The other problem was that discover_opts() also mishandled relative paths.
   When the cursor started out as ".", cursor.pop() would be blindly called
   such that cursor would be "" for the second iteration. When a ceiling
   directory was in use such that there was a max_height, the current height
   would be burned going from ".", to "", and then to "<cwd>", before finally
   actually getting to a real parent directory. This problem is ameliorated by
   testing whether the cursor has a non-empty parent before popping.
   
   N.B. the new test case relies on the test running from the gix-discover
   directory such that the gitoxide repository will be found. This is a bit
   fragile and will fail if, for example, the test is run from an unpacked
   gitoxide source tarball.

### New Features (BREAKING)

 - <csr-id-b4dcfc716a80ffccbab6f7ccc586d8063cc10fff/> `Path::from_dot_git_dir()` now takes the `current_dir` as argument and returns `Option<path>`
   That way it's possible to avoid at least one call of
   `std::env::current_dir()` per invocation, which also is more consistent
   with similar plumbing methods.
   
   Furthermore it can signal with `None` if the input directory was invalid.
 - <csr-id-3d8fa8fef9800b1576beab8a5bc39b821157a5ed/> upgrade edition to 2021 in most crates.
   MSRV for this is 1.56, and we are now at 1.60 so should be compatible.
   This isn't more than a patch release as it should break nobody
   who is adhering to the MSRV, but let's be careful and mark it
   breaking.
   
   Note that `gix-features` and `gix-pack` are still on edition 2018
   as they make use of a workaround to support (safe) mutable access
   to non-overlapping entries in a slice which doesn't work anymore
   in edition 2021.

## 0.8.0 (2022-11-17)

A maintenance release without user-facing changes.

## 0.7.0 (2022-11-06)

A maintenance release without user-facing changes.

## 0.6.0 (2022-10-10)

Maintenance release without user-facing changes.

## 0.5.0 (2022-09-20)

### Changed (BREAKING)

 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`

## 0.4.2 (2022-09-01)

<csr-id-7057ad2251f76756a5b35b6bbba10ab9b6601226/>
<csr-id-08d0c27b0641526aa8b70ee5aadd3658862bef1b/>

### Other

 - <csr-id-7057ad2251f76756a5b35b6bbba10ab9b6601226/> test `is_git` on macOS exfat
 - <csr-id-08d0c27b0641526aa8b70ee5aadd3658862bef1b/> allow opening repos on macos exfat

## 0.4.1 (2022-08-24)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

## 0.4.0 (2022-08-17)

### New Features

 - <csr-id-2e015a3874ab4db40449ec4b8ffb126d8e1a22a4/> add `is_submodule_git_dir()`
 - <csr-id-4a3e1cfbc537963b851c66754e7724875b49b1f4/> `is_git()` now detects submodule worktrees correctly.
 - <csr-id-aa6fd9702e16b4a2bedb0dd0d2323d06133d1a7f/> `gix_discover::is_git()` can detect submodule dirs correctly enough.
   We currently detect them as possibly bare, which could be improved if we
   allow ourselves to see `.git/modules` as `submodule` always.

### Changed (BREAKING)

 - <csr-id-1b0ef1856f542e5dc23310be93c2b39a9d84cb80/> Provide more details when classifying submodule directories.
   That way we can avoid feeding `.git` files to `git-repository::open()`
   and avoid work duplication, which ultimately allows to open submodules
   directories of all kinds.

## 0.3.0 (2022-07-22)

### New Features

 - <csr-id-010350180459aec41132c960ddafc7b81dd9c04d/> add `DOT_GIT_DIR` constant, containing the name ".git".

## 0.2.0 (2022-06-13)

<csr-id-6106521581029c5c24b23a47bb91c1921edfa0af/>

### Other

 - <csr-id-6106521581029c5c24b23a47bb91c1921edfa0af/> avoid shortening absolute paths

### New Features (BREAKING)

 - <csr-id-266d4379e9132fd7dd21e6c8fccb36e125069d6e/> Make `realpath()` easier to use by introducing `realpath_opt()`.
   That way there is consistency about how many symlinks to follow.

## 0.1.3 (2022-05-27)

A maintenance release without user-facing changes.

## 0.1.2 (2022-05-23)

<csr-id-136eb37b00c9e7ba0fd0bc2a75dee2ac1b06516d/>

### Features

- `discover` now avoid crossing file-system boundaries by default on unix.

### Other

 - <csr-id-136eb37b00c9e7ba0fd0bc2a75dee2ac1b06516d/> adopt git-for-windows exception rules

## 0.1.1 (2022-05-21)

<csr-id-e63e722791a7795cd99048bed834459595c60abc/>
<csr-id-277c41f1185398e14f92247aada8422f6b08afd1/>

### Other

 - <csr-id-e63e722791a7795cd99048bed834459595c60abc/> add ceiling_dirs option to upwards discovery

### Other

 - <csr-id-277c41f1185398e14f92247aada8422f6b08afd1/> add cross_fs option to upwards discovery

## 0.1.0 (2022-05-18)

### New Features

 - <csr-id-050f795bfb0fe11655cd7e45c10d87c89ba82625/> allow discovery of linked worktree git dirs
   This also works if the work-tree can't be found but it is otherwise
   a valid git dir.

## 0.0.0 (2022-05-06)

The first release, an empty crate.
