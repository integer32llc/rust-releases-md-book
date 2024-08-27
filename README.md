# Rust Release Notes as an mdbook

This is a proof-of-concept proposal for discussion of possibly rendering the Rust release notes as
an mdbook to be deployed somewhere on doc.rust-lang.org and/or included with Rust releases for local
viewing. Maybe.

[View the rendered release notes mdbook](https://integer32llc.github.io/rust-releases-md-book/).
Note that it is currently being updated by hand, so may not be current with what is in the
rust-lang/rust repository.

## Problems

The canonical source for Rust release notes is [https://github.com/rust-lang/rust/blob/master/RELEASES.md](https://github.com/rust-lang/rust/blob/master/RELEASES.md),
one big, long Markdown file that is difficult to navigate and sometimes even difficult for GitHub
to render completely. The only way to search it is with find-in-page.

There is a [rendered HTML page of the release notes](https://doc.rust-lang.org/releases.html), but
it is also one large hard-to-navigate page, and doesn't have dark mode. I'm also not sure if that's
linked to from anywhere; I don't see it on [https://www.rust-lang.org/learn](https://www.rust-lang.org/learn)
(which is where [doc.rust-lang.org](https://doc.rust-lang.org) redirects to) and I only found it by
looking for release-notes related stuff in the rust-lang/rust repo.

## Proposed solution

A Rust script (a prototype of which is in this repo) that reads the existing RELEASES.md and writes
out mdbook files with one minor release per chapter.

### Advantages

- No need for the release team to change anything about the current release notes process
- Easier to navigate to a particular release
- mdbook provides better search than find-in-page
- mdbook provides dark mode
- mdbooks are already used in Rust documentation

### Disadvantages

- Yet Another Thing to maintain

### Remaining work

- Bikeshed the way the mdbook is implemented, such as how much metadata about a release should be
  displayed in the sidebar
- Integrate the script and mdbook into rust-lang/rust's current mdbook processes
