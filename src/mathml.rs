//! `MathML` element factory functions.
//!
//! Provides factory functions for `MathML` elements (e.g. `mi`, `mn`, `mrow`).
//! All functions return [`XmlElement`](crate::XmlElement), which self-closes when
//! children are empty.
//!
//! This module is enabled by the `mathml` feature (default on).

use crate::define_xml_elements;

define_xml_elements! {
    mi => "mi",
    mn => "mn",
    mo => "mo",
    mtext => "mtext",
    mspace => "mspace",
    ms => "ms",
    mrow => "mrow",
    mfrac => "mfrac",
    msqrt => "msqrt",
    mroot => "mroot",
    mstyle => "mstyle",
    merror => "merror",
    mpadded => "mpadded",
    mphantom => "mphantom",
    mfenced => "mfenced",
    mline => "mline",
    menclose => "menclose",
    msub => "msub",
    msup => "msup",
    msubsup => "msubsup",
    munder => "munder",
    mover => "mover",
    munderover => "munderover",
    mmultiscripts => "mmultiscripts",
    mtable => "mtable",
    mtr => "mtr",
    mtd => "mtd",
    mlabeled_tr => "mlabeledtr",
    maction => "maction",
    annotation => "annotation",
    annotation_xml => "annotation-xml",
    semantics => "semantics",
}
