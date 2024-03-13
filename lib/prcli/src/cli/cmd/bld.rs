use clap::{Parser, Subcommand, Args};
use strum::Display;

use super::{Cmd, Subcmd};

#[derive(Default, Parser, Debug)]
#[clap(name = "build", about = "Manage build")]
pub struct BuildCmd {
    #[clap(subcommand)]
    pub cmd: Option<BuildSubCmd>,

}
#[derive(Default, Subcommand, Debug, Display, PartialEq)]
pub enum BuildSubCmd {
    #[clap(name = "id")]
    Id,
    #[clap(name = "bld")]
    Bld,
    #[clap(name = "run")]
    Run,
    #[clap(name = "sh")]
    Sh,
    #[clap(name = "init")]
    Init,
    #[clap(name = "Build")]
    Build,
    #[clap(name = "rm")]
    Rm,
    #[clap(name = "mv")]
    Mv,
    #[clap(name = "cp")]
    Cp,
    #[clap(name = "cat")]
    Cat,
    #[clap(name = "edit")]
    Edit,
    #[clap(name = "show")]
    Show,
    #[clap(name = "hide")]
    Hide,
    #[clap(name = "clone")]
    Clone,
    #[clap(name = "diff")]
    Diff,
    #[clap(name = "merge")]
    Merge,
    #[clap(name = "push")]
    Push,
    #[clap(name = "pull")]
    Pull,
    #[clap(name = "fetch")]
    Fetch,
    #[clap(name = "commit")]
    Commit,
    #[clap(name = "status")]
    Status,
    #[clap(name = "log")]
    Log,
    #[clap(name = "branch")]
    Branch,
    #[clap(name = "checkout")]
    Checkout,
    #[clap(name = "reset")]
    Reset,
    #[clap(name = "rebase")]
    Rebase,
    #[clap(name = "merge-base")]
    MergeBase,
    #[clap(name = "cherry-pick")]
    CherryPick,
    #[clap(name = "revert")]
    Revert,
    #[clap(name = "tag")]
    Tag,
    #[clap(name = "describe")]
    Describe,
    #[clap(name = "show-ref")]
    ShowRef,
    #[clap(name = "rev-parse")]
    RevParse,
    #[clap(name = "for-each-ref")]
    ForEachRef,
    #[clap(name = "update-ref")]
    UpdateRef,
    #[clap(name = "prune")]
    Prune,
    #[clap(name = "reflog")]
    Reflog,
    #[clap(name = "filter-branch")]
    FilterBranch,
    #[clap(name = "submodule")]
    Submodule,
    #[clap(name = "bld-help")]
    #[default]
    Help,

}
impl super::super::Cmd for BuildCmd {
    type Sub = BuildSubCmd;
}
impl Subcmd for BuildSubCmd {
    type Parent = BuildCmd;
}