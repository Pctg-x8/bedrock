//! v1.2 promoted elements

use crate::{
    parts::{Bitmask, Command, Element, Enum, ExtensionHeaderConstants, Struct, StructUsage},
    vk_ext_enum,
};

pub const ELEMENTS: &[Element] = &[
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_KHR_timeline_semaphore", 2)),
    Element::Enum(
        Enum::new(
            "SemaphoreType",
            "SEMAPHORE_TYPE",
            &[
                Enum::member("BINARY", 0)
                    .extension("VK_KHR_timeline_semaphore", "KHR")
                    .promoted("1_2"),
                Enum::member("TIMELINE", 1)
                    .extension("VK_KHR_timeline_semaphore", "KHR")
                    .promoted("1_2"),
            ],
        )
        .extension("VK_KHR_timeline_semaphore", "KHR")
        .promoted("1_2"),
    ),
    Element::Bitmask(
        Bitmask::new(
            "SemaphoreWaitFlags",
            "SemaphoreWaitFlagBits",
            "SEMAPHORE_WAIT",
            &[Bitmask::entry("ANY", 0)
                .extension("KHR", "timeline_semaphore")
                .promoted("1_2")],
        )
        .extension("KHR", "timeline_semaphore")
        .promoted("1_2"),
    ),
    Element::Struct(
        Struct::new(
            "PhysicalDeviceTimelineSemaphoreFeatures",
            &[Struct::member("timelineSemaphore", "VkBool32")],
        )
        .stype(
            "PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES",
            vk_ext_enum(208, 0) as _,
            StructUsage::Both,
        )
        .extensions(&[("KHR", "timeline_semaphore")])
        .promoted("1_2"),
    ),
    Element::Struct(
        Struct::new(
            "PhysicalDeviceTimelineSemaphoreProperties",
            &[Struct::member("maxTimelineSemaphoreValueDifference", "u64")],
        )
        .stype(
            "PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES",
            vk_ext_enum(208, 1) as _,
            StructUsage::Sink,
        )
        .extensions(&[("KHR", "timeline_semaphore")])
        .promoted("1_2"),
    ),
    Element::Struct(
        Struct::new(
            "SemaphoreSignalInfo",
            &[
                Struct::member("semaphore", "VkSemaphore"),
                Struct::member("value", "u64"),
            ],
        )
        .stype("SEMAPHORE_SIGNAL_INFO", vk_ext_enum(208, 5) as _, StructUsage::Source)
        .extensions(&[("KHR", "timeline_semaphore")])
        .promoted("1_2"),
    ),
    Element::Struct(
        Struct::new(
            "SemaphoreTypeCreateInfo",
            &[
                Struct::member("semaphoreType", "VkSemaphoreTypeKHR"),
                Struct::member("initialValue", "u64"),
            ],
        )
        .stype(
            "SEMAPHORE_TYPE_CREATE_INFO",
            vk_ext_enum(208, 2) as _,
            StructUsage::Source,
        )
        .extensions(&[("KHR", "timeline_semaphore")])
        .promoted("1_2"),
    ),
    Element::Struct(
        Struct::new(
            "SemaphoreWaitInfo",
            &[
                Struct::member("flags", "VkSemaphoreWaitFlagsKHR"),
                Struct::member("semaphoreCount", "u32"),
                Struct::member("pSemaphores", "*const VkSemaphore"),
                Struct::member("pValues", "*const u64"),
            ],
        )
        .stype("SEMAPHORE_WAIT_INFO", vk_ext_enum(208, 4) as _, StructUsage::Source)
        .extensions(&[("KHR", "timeline_semaphore")])
        .promoted("1_2"),
    ),
    Element::Struct(
        Struct::new(
            "TimelineSemaphoreSubmitInfo",
            &[
                Struct::member("waitSemaphoreValueCount", "u32"),
                Struct::member("pWaitSemaphoreValues", "*const u64"),
                Struct::member("signalSemaphoreValueCount", "u32"),
                Struct::member("pSignalSemaphoreValues", "*const u64"),
            ],
        )
        .stype(
            "TIMELINE_SEMAPHORE_SUBMIT_INFO",
            vk_ext_enum(208, 3) as _,
            StructUsage::Source,
        )
        .extensions(&[("KHR", "timeline_semaphore")])
        .promoted("1_2"),
    ),
    Element::Command(
        Command::new(
            "GetSemaphoreCounterValue",
            &[
                ("device", "VkDevice"),
                ("semaphore", "VkSemaphore"),
                ("pValue", "*mut u64"),
            ],
        )
        .failable()
        .extension("KHR", "timeline_semaphore")
        .promoted("1_2"),
    ),
    Element::Command(
        Command::new(
            "SignalSemaphore",
            &[
                ("device", "VkDevice"),
                ("pSignalInfo", "*const VkSemaphoreSignalInfoKHR"),
            ],
        )
        .failable()
        .extension("KHR", "timeline_semaphore")
        .promoted("1_2"),
    ),
    Element::Command(
        Command::new(
            "WaitSemaphores",
            &[
                ("device", "VkDevice"),
                ("pWaitInfo", "*const VkSemaphoreWaitInfoKHR"),
                ("timeout", "u64"),
            ],
        )
        .failable()
        .extension("KHR", "timeline_semaphore")
        .promoted("1_2"),
    ),
];
