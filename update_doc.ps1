cargo doc --features=FeImplements,FePresentation,VK_EXT_debug_report
if($?) { robocopy target\doc docs /mir }
