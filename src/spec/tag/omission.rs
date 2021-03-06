use lazy_static::lazy_static;
use std::collections::{HashSet, HashMap};

// Rules sourced from https://html.spec.whatwg.org/multipage/syntax.html#syntax-tag-omission.
// TODO html, head, body
// TODO Opening tags

pub enum ClosingTagOmissionRuleIfLast {
    // Closing tag can always be omitted if it's the last node of its parent's children.
    Always,
    // Closing tag can never be omitted if it's the last node of its parent's children.
    Never,
    // Closing tag can be omitted if it's the last node of its parent's children and the parent tag name is not one of these.
    ParentIsNot(HashSet<&'static [u8]>),
}

pub struct ClosingTagOmissionRule {
    // Closing tag can be omitted if immediately followed by an element node with one of these tag names.
    followed_by: HashSet<&'static [u8]>,
    // Closing tag can be omitted if it's the last node of its parent's children.
    is_last: ClosingTagOmissionRuleIfLast,
}

impl ClosingTagOmissionRule {
    pub fn can_omit_as_last_node(&self, parent: Option<&[u8]>) -> bool {
        match &self.is_last {
            ClosingTagOmissionRuleIfLast::Always => true,
            ClosingTagOmissionRuleIfLast::Never => false,
            ClosingTagOmissionRuleIfLast::ParentIsNot(parents) => match parent {
                Some(tag) => !parents.contains(tag),
                None => true,
            },
        }
    }

    pub fn can_omit_as_before(&self, after: &[u8]) -> bool {
        self.followed_by.contains(after)
    }
}

lazy_static! {
    static ref LI_CLOSING_TAG_OMISSION_RULE: ClosingTagOmissionRule = ClosingTagOmissionRule {
        followed_by: {
            let mut s = HashSet::<&'static [u8]>::new();
            s.insert(b"li");
            s
        },
        is_last: ClosingTagOmissionRuleIfLast::Always,
    };
}

lazy_static! {
    static ref DT_CLOSING_TAG_OMISSION_RULE: ClosingTagOmissionRule = ClosingTagOmissionRule {
        followed_by: {
            let mut s = HashSet::<&'static [u8]>::new();
            s.insert(b"dt");
            s.insert(b"dd");
            s
        },
        is_last: ClosingTagOmissionRuleIfLast::Never,
    };
}

lazy_static! {
    static ref DD_CLOSING_TAG_OMISSION_RULE: ClosingTagOmissionRule = ClosingTagOmissionRule {
        followed_by: {
            let mut s = HashSet::<&'static [u8]>::new();
            s.insert(b"dd");
            s.insert(b"dt");
            s
        },
        is_last: ClosingTagOmissionRuleIfLast::Always,
    };
}

lazy_static! {
    static ref P_CLOSING_TAG_OMISSION_RULE: ClosingTagOmissionRule = {
        let mut followed_by = HashSet::<&'static [u8]>::new();
        followed_by.insert(b"address");
        followed_by.insert(b"article");
        followed_by.insert(b"aside");
        followed_by.insert(b"blockquote");
        followed_by.insert(b"details");
        followed_by.insert(b"div");
        followed_by.insert(b"dl");
        followed_by.insert(b"fieldset");
        followed_by.insert(b"figcaption");
        followed_by.insert(b"figure");
        followed_by.insert(b"footer");
        followed_by.insert(b"form");
        followed_by.insert(b"h1");
        followed_by.insert(b"h2");
        followed_by.insert(b"h3");
        followed_by.insert(b"h4");
        followed_by.insert(b"h5");
        followed_by.insert(b"h6");
        followed_by.insert(b"header");
        followed_by.insert(b"hgroup");
        followed_by.insert(b"hr");
        followed_by.insert(b"main");
        followed_by.insert(b"menu");
        followed_by.insert(b"nav");
        followed_by.insert(b"ol");
        followed_by.insert(b"p");
        followed_by.insert(b"pre");
        followed_by.insert(b"section");
        followed_by.insert(b"table");
        followed_by.insert(b"ul");

        let mut is_last_tags = HashSet::<&'static [u8]>::new();
        is_last_tags.insert(b"a");
        is_last_tags.insert(b"audio");
        is_last_tags.insert(b"del");
        is_last_tags.insert(b"ins");
        is_last_tags.insert(b"map");
        is_last_tags.insert(b"noscript");
        is_last_tags.insert(b"video");

        ClosingTagOmissionRule { followed_by, is_last: ClosingTagOmissionRuleIfLast::ParentIsNot(is_last_tags) }
    };
}

lazy_static! {
    static ref RT_CLOSING_TAG_OMISSION_RULE: ClosingTagOmissionRule = ClosingTagOmissionRule {
        followed_by: {
            let mut s = HashSet::<&'static [u8]>::new();
            s.insert(b"rt");
            s.insert(b"rp");
            s
        },
        is_last: ClosingTagOmissionRuleIfLast::Always,
    };
}

lazy_static! {
    static ref RP_CLOSING_TAG_OMISSION_RULE: ClosingTagOmissionRule = ClosingTagOmissionRule {
        followed_by: {
            let mut s = HashSet::<&'static [u8]>::new();
            s.insert(b"rt");
            s.insert(b"rp");
            s
        },
        is_last: ClosingTagOmissionRuleIfLast::Always,
    };
}

lazy_static! {
    static ref OPTGROUP_CLOSING_TAG_OMISSION_RULE: ClosingTagOmissionRule = ClosingTagOmissionRule {
        followed_by: {
            let mut s = HashSet::<&'static [u8]>::new();
            s.insert(b"optgroup");
            s
        },
        is_last: ClosingTagOmissionRuleIfLast::Always,
    };
}

lazy_static! {
    static ref OPTION_CLOSING_TAG_OMISSION_RULE: ClosingTagOmissionRule = ClosingTagOmissionRule {
        followed_by: {
            let mut s = HashSet::<&'static [u8]>::new();
            s.insert(b"option");
            s.insert(b"optgroup");
            s
        },
        is_last: ClosingTagOmissionRuleIfLast::Always,
    };
}

lazy_static! {
    static ref THEAD_CLOSING_TAG_OMISSION_RULE: ClosingTagOmissionRule = ClosingTagOmissionRule {
        followed_by: {
            let mut s = HashSet::<&'static [u8]>::new();
            s.insert(b"tbody");
            s.insert(b"tfoot");
            s
        },
        is_last: ClosingTagOmissionRuleIfLast::Never,
    };
}

lazy_static! {
    static ref TBODY_CLOSING_TAG_OMISSION_RULE: ClosingTagOmissionRule = ClosingTagOmissionRule {
        followed_by: {
            let mut s = HashSet::<&'static [u8]>::new();
            s.insert(b"tbody");
            s.insert(b"tfoot");
            s
        },
        is_last: ClosingTagOmissionRuleIfLast::Always,
    };
}

lazy_static! {
    static ref TFOOT_CLOSING_TAG_OMISSION_RULE: ClosingTagOmissionRule = ClosingTagOmissionRule {
        followed_by: HashSet::<&'static [u8]>::new(),
        is_last: ClosingTagOmissionRuleIfLast::Always,
    };
}

lazy_static! {
    static ref TR_CLOSING_TAG_OMISSION_RULE: ClosingTagOmissionRule = ClosingTagOmissionRule {
        followed_by: {
            let mut s = HashSet::<&'static [u8]>::new();
            s.insert(b"tr");
            s
        },
        is_last: ClosingTagOmissionRuleIfLast::Always,
    };
}

lazy_static! {
    static ref TD_CLOSING_TAG_OMISSION_RULE: ClosingTagOmissionRule = ClosingTagOmissionRule {
        followed_by: {
            let mut s = HashSet::<&'static [u8]>::new();
            s.insert(b"td");
            s.insert(b"th");
            s
        },
        is_last: ClosingTagOmissionRuleIfLast::Always,
    };
}

lazy_static! {
    static ref TH_CLOSING_TAG_OMISSION_RULE: ClosingTagOmissionRule = ClosingTagOmissionRule {
        followed_by: {
            let mut s = HashSet::<&'static [u8]>::new();
            s.insert(b"td");
            s.insert(b"th");
            s
        },
        is_last: ClosingTagOmissionRuleIfLast::Always,
    };
}

lazy_static! {
    pub static ref CLOSING_TAG_OMISSION_RULES: HashMap<&'static [u8], &'static ClosingTagOmissionRule> = {
        let mut m = HashMap::<&'static [u8], &'static ClosingTagOmissionRule>::new();
        m.insert(b"li", &LI_CLOSING_TAG_OMISSION_RULE);
        m.insert(b"dt", &DT_CLOSING_TAG_OMISSION_RULE);
        m.insert(b"dd", &DD_CLOSING_TAG_OMISSION_RULE);
        m.insert(b"p", &P_CLOSING_TAG_OMISSION_RULE);
        m.insert(b"rt", &RT_CLOSING_TAG_OMISSION_RULE);
        m.insert(b"rp", &RP_CLOSING_TAG_OMISSION_RULE);
        m.insert(b"optgroup", &OPTGROUP_CLOSING_TAG_OMISSION_RULE);
        m.insert(b"option", &OPTION_CLOSING_TAG_OMISSION_RULE);
        m.insert(b"thead", &THEAD_CLOSING_TAG_OMISSION_RULE);
        m.insert(b"tbody", &TBODY_CLOSING_TAG_OMISSION_RULE);
        m.insert(b"tfoot", &TFOOT_CLOSING_TAG_OMISSION_RULE);
        m.insert(b"tr", &TR_CLOSING_TAG_OMISSION_RULE);
        m.insert(b"td", &TD_CLOSING_TAG_OMISSION_RULE);
        m.insert(b"th", &TH_CLOSING_TAG_OMISSION_RULE);
        m
    };
}
