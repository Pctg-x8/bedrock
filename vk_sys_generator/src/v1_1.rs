//! v1.2 promoted elements

use crate::{parts::*, vk_ext_enum};

const VERSION: &str = "1_1";
const VK_EXT_SHADER_SUBGROUP_VOTE: &Extension = &Extension::ext("shader_subgroup_vote", 1);

pub const ELEMENTS: &[Element] = &[Element::ExtensionHeaderConstants2(ExtensionHeaderConstants2(
    VK_EXT_SHADER_SUBGROUP_VOTE,
))];
