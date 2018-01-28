cargo doc --features=FeImplements,FePresentation,VK_EXT_debug_report
if($?) { robocopy target\doc docs /mir }
mkdir docs/ja
robocopy target\doc docs\ja /mir
node translate_ja.js
