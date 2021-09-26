# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

## Added
- add `trace-libgit` feature to make git tracing optional [[@dm9pZCAq](https://github.com/dm9pZCAq)] ([#902](https://github.com/extrawurst/gitui/issues/902))

## Fixed
- appropriate error message when pulling deleted remote branch ([#911](https://github.com/extrawurst/gitui/issues/991))

## [0.17.1] - 2021-09-10

**fuzzy find files**

![fuzzy-find](assets/fuzzy-find.gif)

**emojified commit message**

![emojified-commit-message](assets/emojified-commit-message.png)

## Added
- add supporting rebasing on branch (if conflict-free) ([#816](https://github.com/extrawurst/gitui/issues/816))
- fuzzy find files ([#891](https://github.com/extrawurst/gitui/issues/891))
- visualize progress during async syntax highlighting ([#889](https://github.com/extrawurst/gitui/issues/889))
- added support for markdown emoji's in commits [[@andrewpollack](https://github.com/andrewpollack)] ([#768](https://github.com/extrawurst/gitui/issues/768))
- added scrollbar to revlog [[@ashvin021](https://github.com/ashvin021)] ([#868](https://github.com/extrawurst/gitui/issues/868))

## Fixed
- fix build when system level libgit2 version was used ([#883](https://github.com/extrawurst/gitui/issues/883))
- fix merging branch not closing branch window [[@andrewpollack](https://github.com/andrewpollack)] ([#876](https://github.com/extrawurst/gitui/issues/876))
- fix commit msg being broken inside tag list ([#871](https://github.com/extrawurst/gitui/issues/871))
- fix filetree file content not showing tabs correctly ([#874](https://github.com/extrawurst/gitui/issues/874))

## Key binding notes
- new keys: `rebase_branch` [`R`], `file_find` [`f`]

see `vim_style_key_config.ron` for their default vim binding

## [0.17.0] - 2021-08-21

**compare commits**

![compare](assets/compare.gif)

**options**

![options](assets/options.gif)

**drop multiple stashes**

![drop-multiple-stashes](assets/drop-multiple-stashes.gif)

**branch name validation**

![name-validation](assets/branch-validation.gif)

## Added
- allow inspecting top commit of a branch from list
- compare commits in revlog and head against branch ([#852](https://github.com/extrawurst/gitui/issues/852))
- new options popup (show untracked files, diff settings) ([#849](https://github.com/extrawurst/gitui/issues/849))
- mark and drop multiple stashes ([#854](https://github.com/extrawurst/gitui/issues/854))
- check branch name validity while typing ([#559](https://github.com/extrawurst/gitui/issues/559))
- support deleting remote branch [[@zcorniere](https://github.com/zcorniere)] ([#622](https://github.com/extrawurst/gitui/issues/622))
- mark remote branches that have local tracking branch [[@jedel1043](https://github.com/jedel1043)] ([#861](https://github.com/extrawurst/gitui/issues/861))

## Fixed
- error viewing filetree in empty repo ([#859](https://github.com/extrawurst/gitui/issues/859))
- do not allow to ignore .gitignore files ([#825](https://github.com/extrawurst/gitui/issues/825))
- crash in shallow repo ([#836](https://github.com/extrawurst/gitui/issues/836))
- fixed performance regression in revlog ([#850](https://github.com/extrawurst/gitui/issues/850))
- fixed performance degradation when quitting on Windows ([#823](https://github.com/extrawurst/gitui/issues/823))

## [0.16.2] - 2021-07-10

**undo last commit**

![undo-last-commit](assets/undo-last-commit.gif)

**mark local tags**

![tag-remote-marker](assets/tag-remote-marker.gif)

## Added
- taglist: show arrow-symbol on tags not present on origin [[@cruessler](https://github.com/cruessler)] ([#776](https://github.com/extrawurst/gitui/issues/776))
- new `undo-last-commit` command [[@remique](https://github.com/remique)] ([#758](https://github.com/extrawurst/gitui/issues/758))
- new quit key `[q]` ([#771](https://github.com/extrawurst/gitui/issues/771))
- proper error message if remote rejects force push ([#801](https://github.com/extrawurst/gitui/issues/801))

## Fixed
- openssl vendoring broken on macos ([#772](https://github.com/extrawurst/gitui/issues/772))
- amend and other commands not shown in help ([#778](https://github.com/extrawurst/gitui/issues/778))
- focus locked on commit msg details in narrow term sizes ([#780](https://github.com/extrawurst/gitui/issues/780))
- non-utf8 file/path names broke filetree ([#802](https://github.com/extrawurst/gitui/issues/802))

## [0.16.1] - 2021-06-06

## Added
- honor `config.showUntrackedFiles` improving speed with a lot of untracked items ([#752](https://github.com/extrawurst/gitui/issues/752))
- improve performance when opening filetree-tab ([#756](https://github.com/extrawurst/gitui/issues/756))
- indicator for longer commit message than displayed ([#773](https://github.com/extrawurst/gitui/issues/773))

![msg-len](assets/long-msg-indicator.gif)
  
## Fixed
- wrong file with same name shown in file tree ([#748](https://github.com/extrawurst/gitui/issues/748))
- filetree collapsing broken on windows ([#761](https://github.com/extrawurst/gitui/issues/761))
- unnecessary overdraw of the spinner on each redraw ([#764](https://github.com/extrawurst/gitui/issues/764))

### Internal
- use git_repository_message [[@kosayoda](https://github.com/kosayoda)] ([#751](https://github.com/extrawurst/gitui/issues/751))

## [0.16.0] - 2021-05-28

**merge branch, merge commit**

![merge-commit](assets/merge-commit-abort.gif)

**tag list popup**

![tagslist](assets/tags-list-popup.gif)

**revision file tree**

![filetree](assets/revision-file-tree.gif)

**commit subject length warning**

![warning](assets/commit-msg-length-limit.gif)

## Added
- merging branches, pull-merge with conflicts, commit merges ([#485](https://github.com/extrawurst/gitui/issues/485))
- tags-list-popup (delete-tag, go to tagged commit) [[@cruessler](https://github.com/cruessler)] ([#483](https://github.com/extrawurst/gitui/issues/483))
- inspect file tree tab ([#743](https://github.com/extrawurst/gitui/issues/743))
- file tree popup (for a specific revision) ([#714](https://github.com/extrawurst/gitui/issues/714))
- warning if commit subject line gets too long ([#478](https://github.com/extrawurst/gitui/issues/478))
- `--bugreport` cmd line arg to help diagnostics [[@zcorniere](https://github.com/zcorniere)] ([#695](https://github.com/extrawurst/gitui/issues/695))

## Changed
- smarter log timestamps ([#682](https://github.com/extrawurst/gitui/issues/682)) 
- create-branch popup aligned with rename-branch [[@bruceCoelho](https://github.com/bruceCoelho)] ([#679](https://github.com/extrawurst/gitui/issues/679))
- smart focus change after staging all files ([#706](https://github.com/extrawurst/gitui/issues/706))
- do not allow to commit when `gpgsign` enabled ([#740](https://github.com/extrawurst/gitui/issues/740))

## Fixed
- selected-tab color broken in light theme [[@Cottser](https://github.com/Cottser)] ([#719](https://github.com/extrawurst/gitui/issues/719))
- proper tmp file location to externally edit commit msg ([#518](https://github.com/extrawurst/gitui/issues/518))

## [0.15.0] - 2021-04-27

**file blame**

![blame](assets/blame.gif)

### Added
- blame a file [[@cruessler](https://github.com/cruessler)] ([#484](https://github.com/extrawurst/gitui/issues/484))
- support commit.template [[@wandernauta](https://github.com/wandernauta)] ([#546](https://github.com/extrawurst/gitui/issues/546))

### Fixed
- debug print when adding a file to ignore
- fix scrolling long messages in commit details view ([#663](https://github.com/extrawurst/gitui/issues/663))
- limit log messages in log tab ([#652](https://github.com/extrawurst/gitui/issues/652))
- fetch crashed when no upstream of branch is set ([#637](https://github.com/extrawurst/gitui/issues/637))
- `enter` key panics in empty remote branch list ([#643](https://github.com/extrawurst/gitui/issues/643))

### Internal
- cleanup some stringly typed code [[@wandernauta](https://github.com/wandernauta)] ([#655](https://github.com/extrawurst/gitui/issues/655))
- introduce EventState enum (removing bool for even propagation) [[@tisorlawan](https://github.com/tisorlawan)] ([#665](https://github.com/extrawurst/gitui/issues/665))

## [0.14.0] - 2021-04-11

### Added
- `[w]` key to toggle between staging/workdir [[@terhechte](https://github.com/terhechte)] ([#595](https://github.com/extrawurst/gitui/issues/595))
- view/checkout remote branches ([#617](https://github.com/extrawurst/gitui/issues/617))

![checkout-remote](assets/checkout-remote.gif)

### Changed
- ask to pop stash by default (*apply* using `[a]` now) [[@brunogouveia](https://github.com/brunogouveia)] ([#574](https://github.com/extrawurst/gitui/issues/574))

![stash_pop](assets/stash_pop.gif)

### Fixed
- push branch to its tracking remote ([#597](https://github.com/extrawurst/gitui/issues/597))
- fixed panic when staging lines involving missing newline eof ([#605](https://github.com/extrawurst/gitui/issues/605))
- fixed pull/fetch deadlocking when it fails ([#624](https://github.com/extrawurst/gitui/issues/624))

## [0.13.0] - 2021-03-15 - Happy Birthday GitUI 🥳

Thanks for your interest and support over this year! Read more about the 1 year anniversary reflections of this project on my [blog](https://blog.extrawurst.org/general/programming/rust/2021/03/15/gitui-a-year-in-opensource.html).

**stage/unstage/discard by line**

![by-line-ops](assets/by-line-ops.gif)

**push tags**

![push-tags](assets/push_tags.gif)

### Changed
- `[s]` key repurposed to trigger line based (un)stage
- cleanup status/diff commands to be more context sensitive ([#572](https://github.com/extrawurst/gitui/issues/572))

### Added
- support pull via rebase (using config `pull.rebase`) ([#566](https://github.com/extrawurst/gitui/issues/566))
- support stage/unstage selected lines ([#59](https://github.com/extrawurst/gitui/issues/59))
- support discarding selected lines ([#59](https://github.com/extrawurst/gitui/issues/59))
- support for pushing tags ([#568](https://github.com/extrawurst/gitui/issues/568))
- visualize *conflicted* files differently ([#576](https://github.com/extrawurst/gitui/issues/576))

### Fixed
- keep diff line selection after staging/unstaging/discarding ([#583](https://github.com/extrawurst/gitui/issues/583))
- fix pull deadlocking when aborting credentials input ([#586](https://github.com/extrawurst/gitui/issues/586))
- error diagnostics for config loading ([#589](https://github.com/extrawurst/gitui/issues/589))

## [0.12.0] - 2021-03-03

**pull support (ff-merge or conflict-free merge-commit)**

![pull](assets/pull.gif)

**more info in commit popup**

![chars-branch-name](assets/chars_and_branchname.gif)

### Breaking Change
- MacOS config directory now uses `~/.config/gitui` [[@remique](https://github.com/remique)] ([#317](https://github.com/extrawurst/gitui/issues/317)) 

### Added
- support for pull (fetch + simple merging) ([#319](https://github.com/extrawurst/gitui/issues/319))
- show used char count in input texts ([#466](https://github.com/extrawurst/gitui/issues/466))
- support smoother left/right toggle/keys for commit details ([#418](https://github.com/extrawurst/gitui/issues/418))
- support *force push* command [[@WizardOhio24](https://github.com/WizardOhio24)] ([#274](https://github.com/extrawurst/gitui/issues/274)) 

### Fixed
- don't close branchlist every time ([#550](https://github.com/extrawurst/gitui/issues/550))
- fixed key binding for *external exitor* in vim key bindings [[@yanganto](https://github.com/yanganto)] ([#549](https://github.com/extrawurst/gitui/issues/549))
- fix some potential errors when deleting files while they are being diffed ([#490](https://github.com/extrawurst/gitui/issues/490))
- push defaults to 'origin' remote if it exists ([#494](https://github.com/extrawurst/gitui/issues/494))
- support missing pageUp/down support in branchlist ([#519](https://github.com/extrawurst/gitui/issues/519))
- don't hide branch name while in commit dialog ([#529](https://github.com/extrawurst/gitui/issues/529))
- don't discard commit message without confirmation ([#530](https://github.com/extrawurst/gitui/issues/530))
- compilation broken on freebsd ([#461](https://github.com/extrawurst/gitui/issues/461))
- don’t fail if `user.name` is not set [[@cruessler](https://github.com/cruessler)] ([#79](https://github.com/extrawurst/gitui/issues/79)) ([#228](https://github.com/extrawurst/gitui/issues/228))

## [0.11.0] - 2021-12-20

### Added
- push to remote ([#265](https://github.com/extrawurst/gitui/issues/265)) ([#267](https://github.com/extrawurst/gitui/issues/267))

![push](assets/push.gif)

- number of incoming/outgoing commits to upstream ([#362](https://github.com/extrawurst/gitui/issues/362))
- new branch list popup incl. checkout/delete/rename [[@WizardOhio24](https://github.com/WizardOhio24)] ([#303](https://github.com/extrawurst/gitui/issues/303)) ([#323](https://github.com/extrawurst/gitui/issues/323))

![branches](assets/branches.gif)

- compact treeview [[@WizardOhio24](https://github.com/WizardOhio24)] ([#192](https://github.com/extrawurst/gitui/issues/192))

![tree](assets/compact-tree.png)

- scrollbar in long commit messages [[@timaliberdov](https://github.com/timaliberdov)] ([#308](https://github.com/extrawurst/gitui/issues/308))
- added windows scoop recipe ([#164](https://github.com/extrawurst/gitui/issues/164))
- added gitui to [chocolatey](https://chocolatey.org/packages/gitui) on windows by [@nils-a](https://github.com/nils-a)
- added gitui gentoo instructions to readme [[@dm9pZCAq](https://github.com/dm9pZCAq)] ([#430](https://github.com/extrawurst/gitui/pull/430))
- added windows installer (msi) to release [[@pm100](https://github.com/pm100)] ([#360](https://github.com/extrawurst/gitui/issues/360))
- command to copy commit hash [[@yanganto](https://github.com/yanganto)] ([#281](https://github.com/extrawurst/gitui/issues/281))

### Changed
- upgrade `dirs` to `dirs-next` / remove cfg migration code ([#351](https://github.com/extrawurst/gitui/issues/351)) ([#366](https://github.com/extrawurst/gitui/issues/366))
- do not highlight selection in diff view when not focused ([#270](https://github.com/extrawurst/gitui/issues/270))
- copy to clipboard using `xclip`(linux), `pbcopy`(mac) or `clip`(win) [[@cruessler](https://github.com/cruessler)] ([#262](https://github.com/extrawurst/gitui/issues/262))

### Fixed
- crash when changing git repo while gitui is open ([#271](https://github.com/extrawurst/gitui/issues/271))
- remove workaround for color serialization [[@1wilkens](https://github.com/1wilkens)] ([#149](https://github.com/extrawurst/gitui/issues/149))
- crash on small terminal size ([#307](https://github.com/extrawurst/gitui/issues/307))
- fix vim keybindings uppercase handling [[@yanganto](https://github.com/yanganto)] ([#286](https://github.com/extrawurst/gitui/issues/286))
- remove shift tab windows workaround [[@nils-a](https://github.com/nils-a)] ([#112](https://github.com/extrawurst/gitui/issues/112))
- core.editor is ignored [[@pm100](https://github.com/pm100)] ([#414](https://github.com/extrawurst/gitui/issues/414))

## [0.10.1] - 2020-09-01

### Fixed
- static linux binaries broke due to new clipboard feature which is disabled on linux for now ([#259](https://github.com/extrawurst/gitui/issues/259))

## [0.10.0] - 2020-08-29

### Added

- fully **customizable key bindings** (see [KEY_CONFIG.md](KEY_CONFIG.md)) [[@yanganto](https://github.com/yanganto)] ([#109](https://github.com/extrawurst/gitui/issues/109)) ([#57](https://github.com/extrawurst/gitui/issues/57))
- support scrolling in long commit messages [[@cruessler](https://github.com/cruessler)]([#208](https://github.com/extrawurst/gitui/issues/208))

![scrolling](assets/msg-scrolling.gif)

- copy lines from diffs to clipboard [[@cruessler](https://github.com/cruessler)]([#229](https://github.com/extrawurst/gitui/issues/229))

![select-copy](assets/select-copy.gif)

- scrollbar in long diffs ([#204](https://github.com/extrawurst/gitui/issues/204))

![scrollbar](assets/scrollbar.gif)

- allow creating new branch ([#253](https://github.com/extrawurst/gitui/issues/253))

### Fixed

- selection error in stashlist when deleting last element ([#223](https://github.com/extrawurst/gitui/issues/223))
- git hooks broke ci build on windows [[@dr-BEat](https://github.com/dr-BEat)] ([#235](https://github.com/extrawurst/gitui/issues/235))

## [0.9.1] - 2020-07-30

### Added

- move to (un)staged when the current selection is empty [[@jonstodle](https://github.com/jonstodle)]([#215](https://github.com/extrawurst/gitui/issues/215))
- pending load of a diff/status is visualized ([#160](https://github.com/extrawurst/gitui/issues/160))
- entry on [git-scm.com](https://git-scm.com/downloads/guis) in the list of GUI tools [[@Vidar314](https://github.com/Vidar314)] (see [PR](https://github.com/git/git-scm.com/pull/1485))
- commits can be tagged in revlog [[@cruessler](https://github.com/cruessler)]([#103](https://github.com/extrawurst/gitui/issues/103))

![](assets/tagging.gif)

### Changed

- async fetching tags to improve reactivity in giant repos ([#170](https://github.com/extrawurst/gitui/issues/170))

### Fixed

- removed unmaintained dependency `spin` ([#172](https://github.com/extrawurst/gitui/issues/172))
- opening relative paths in external editor may fail in subpaths ([#184](https://github.com/extrawurst/gitui/issues/184))
- crashes in revlog with utf8 commit messages ([#188](https://github.com/extrawurst/gitui/issues/188))
- `add_to_ignore` failed on files without a newline at EOF ([#191](https://github.com/extrawurst/gitui/issues/191))
- new tags were not picked up in revlog view ([#190](https://github.com/extrawurst/gitui/issues/190))
- tags not shown in commit details popup ([#193](https://github.com/extrawurst/gitui/issues/193))
- min size for relative popups on small terminals ([#179](https://github.com/extrawurst/gitui/issues/179))
- fix crash on resizing terminal to very small width ([#198](https://github.com/extrawurst/gitui/issues/198))
- fix broken tags when using a different internal representation ([#206](https://github.com/extrawurst/gitui/issues/206))
- tags are not cleanly seperated in details view ([#212](https://github.com/extrawurst/gitui/issues/212))

## [0.8.1] - 2020-07-07

### Added

- open file in editor [[@jonstodle](https://github.com/jonstodle)]([#166](https://github.com/extrawurst/gitui/issues/166))

### Fixed

- switch deprecated transitive dependency `net2`->`socket2` [in `crossterm`->`mio`]([#66](https://github.com/extrawurst/gitui/issues/66))
- crash diffing a stash that was created via cli ([#178](https://github.com/extrawurst/gitui/issues/178))
- zero delta file size in diff of untracked binary file ([#171](https://github.com/extrawurst/gitui/issues/171))
- newlines not visualized correctly in commit editor ([#169](https://github.com/extrawurst/gitui/issues/169))

![](assets/newlines.gif)

## [0.8.0] - 2020-07-06

### Added

- core homebrew [formulae](https://formulae.brew.sh/formula/gitui#default): `brew install gitui` [[@vladimyr](https://github.com/vladimyr)](<[#137](https://github.com/extrawurst/gitui/issues/137)>)
- show file sizes and delta on binary diffs ([#141](https://github.com/extrawurst/gitui/issues/141))

![](assets/binary_diff.png)

- external editor support for commit messages [[@jonstodle](https://github.com/jonstodle)](<[#46](https://github.com/extrawurst/gitui/issues/46)>)

![](assets/vi_support.gif)

### Changed

- use terminal blue as default selection background ([#129](https://github.com/extrawurst/gitui/issues/129))
- author column in revlog is now fixed width for better alignment ([#148](https://github.com/extrawurst/gitui/issues/148))
- cleaner tab bar and background work indicating spinner:

![](assets/spinner.gif)

### Fixed

- clearer help headers ([#131](https://github.com/extrawurst/gitui/issues/131))
- display non-utf8 commit messages at least partially ([#150](https://github.com/extrawurst/gitui/issues/150))
- hooks ignored when running `gitui` in subfolder of workdir ([#151](https://github.com/extrawurst/gitui/issues/151))
- better scrolling in file-trees [[@tisorlawan](https://github.com/tisorlawan)]([#144](https://github.com/extrawurst/gitui/issues/144))
- show untracked files in stash commit details [[@MCord](https://github.com/MCord)]([#130](https://github.com/extrawurst/gitui/issues/130))
- in some repos looking up the branch name was a bottleneck ([#159](https://github.com/extrawurst/gitui/issues/159))
- some optimizations in reflog
- fix arrow utf8 encoding in help window [[@daober](https://github.com/daober)]([#142](https://github.com/extrawurst/gitui/issues/142))

## [0.7.0] - 2020-06-15

### Added

- Inspect stash commit in detail ([#121](https://github.com/extrawurst/gitui/issues/121))
- Support reset/revert individual hunks ([#11](https://github.com/extrawurst/gitui/issues/11))
- Commit Amend (`ctrl+a`) when in commit popup ([#89](https://github.com/extrawurst/gitui/issues/89))

![](assets/amend.gif)

### Changed

- file trees: `arrow-right` on expanded folder moves down into folder
- better scrolling in diff ([#52](https://github.com/extrawurst/gitui/issues/52))
- display current branch in status/log ([#115](https://github.com/extrawurst/gitui/issues/115))
- commit msg popup: add cursor and more controls (`arrow-left/right`, `delete` & `backspace`) [[@alistaircarscadden](https://github.com/alistaircarscadden)]([#46](https://github.com/extrawurst/gitui/issues/46))
- moved `theme.ron` from `XDG_CACHE_HOME` to `XDG_CONFIG_HOME` [[@jonstodle](https://github.com/jonstodle)](<[#98](https://github.com/extrawurst/gitui/issues/98)>)

### Fixed

- reset file inside folder failed when running `gitui` in a subfolder too ([#118](https://github.com/extrawurst/gitui/issues/118))
- selection could disappear into collapsed folder ([#120](https://github.com/extrawurst/gitui/issues/120))
- `Files: loading` sometimes wrong ([#119](https://github.com/extrawurst/gitui/issues/119))

## [0.6.0] - 2020-06-09

![](assets/commit-details.gif)

### Changed

- changed hotkeys for selecting stage/workdir (**Note:** use `[w]`/`[s]` to change between workdir and stage) and added hotkeys (`[1234]`) to switch to tabs directly ([#92](https://github.com/extrawurst/gitui/issues/92))
- `arrow-up`/`down` on bottom/top of status file list switches focus ([#105](https://github.com/extrawurst/gitui/issues/105))
- highlight tags in revlog better

### Added

- New `Stage all [a]`/`Unstage all [a]` in changes lists ([#82](https://github.com/extrawurst/gitui/issues/82))
- add `-d`, `--directory` options to set working directory via program arg [[@alistaircarscadden](https://github.com/alistaircarscadden)]([#73](https://github.com/extrawurst/gitui/issues/73))
- commit detail view in revlog ([#80](https://github.com/extrawurst/gitui/issues/80))

### Fixed

- app closes when staging invalid file/path ([#108](https://github.com/extrawurst/gitui/issues/108))
- `shift+tab` not working on windows [[@MCord](https://github.com/MCord)]([#111](https://github.com/extrawurst/gitui/issues/111))

## [0.5.0] - 2020-06-01

### Changed

- support more commands allowing optional multiline commandbar ([#83](https://github.com/extrawurst/gitui/issues/83))

![](assets/cmdbar.gif)

### Added

- support adding untracked file/folder to `.gitignore` ([#44](https://github.com/extrawurst/gitui/issues/44))
- support reverse tabbing using shift+tab ([#92](https://github.com/extrawurst/gitui/issues/92))
- switch to using cmd line args instead of `ENV` (`-l` for logging and `--version`) **please convert your GITUI_LOGGING usage** [[@shenek](https://github.com/shenek)]([#88](https://github.com/extrawurst/gitui/issues/88))
- added missing LICENSE.md files in sub-crates [[@ignatenkobrain](https://github.com/ignatenkobrain)]([#94](https://github.com/extrawurst/gitui/pull/94))

### Fixed

- error when diffing huge files ([#96](https://github.com/extrawurst/gitui/issues/96))
- expressive error when run in bare repos ([#100](https://github.com/extrawurst/gitui/issues/100))

## [0.4.0] - 2020-05-25

### Added

- stashing support (save,apply,drop) ([#3](https://github.com/extrawurst/gitui/issues/3))

### Changed

- log tab refreshes when head changes ([#78](https://github.com/extrawurst/gitui/issues/78))
- performance optimization of the log tab in big repos
- more readable default color for the commit hash in the log tab
- more error/panic resiliance (`unwrap`/`panic` denied by clippy now) [[@MCord](https://github.com/MCord)](<[#77](https://github.com/extrawurst/gitui/issues/77)>)

### Fixes

- panic on small terminal width ([#72](https://github.com/extrawurst/gitui/issues/72))

![](assets/stashing.gif)

## [0.3.0] - 2020-05-20

### Added

- support color themes and light mode [[@MCord](https://github.com/MCord)]([#28](https://github.com/extrawurst/gitui/issues/28))

### Changed

- more natural scrolling in log tab ([#52](https://github.com/extrawurst/gitui/issues/52))

### Fixed

- crash on commit when git name was not set ([#74](https://github.com/extrawurst/gitui/issues/74))
- log tab shown empty in single commit repos ([#75](https://github.com/extrawurst/gitui/issues/75))

![](assets/light-theme.png)

## [0.2.6] - 2020-05-18

### Fixed

- fix crash help in small window size ([#63](https://github.com/extrawurst/gitui/issues/63))

## [0.2.5] - 2020-05-16

### Added

- introduced proper changelog
- hook support on windows [[@MCord](https://github.com/MCord)]([#14](https://github.com/extrawurst/gitui/issues/14))

### Changed

- show longer commit messages in log view
- introduce propper error handling in `asyncgit` [[@MCord](https://github.com/MCord)]([#53](https://github.com/extrawurst/gitui/issues/53))
- better error message when trying to run outside of a valid git repo ([#56](https://github.com/extrawurst/gitui/issues/56))
- improve ctrl+c handling so it is checked first and no component needs to worry of blocking it

### Fixed

- support multiple tags per commit in log ([#61](https://github.com/extrawurst/gitui/issues/61))

## [0.2.3] - 2020-05-12

### Added

- support more navigation keys: home/end/pageUp/pageDown ([#43](https://github.com/extrawurst/gitui/issues/43))
- highlight current tab a bit better

## [0.2.2] - 2020-05-10

### Added

- show tags in commit log ([#47](https://github.com/extrawurst/gitui/issues/47))
- support home/end key in diff ([#43](https://github.com/extrawurst/gitui/issues/43))

### Changed

- close application shortcut is now the standard `ctrl+c`
- some diff improvements ([#42](https://github.com/extrawurst/gitui/issues/42))

### Fixed

- document tab key to switch tabs ([#48](https://github.com/extrawurst/gitui/issues/48))
