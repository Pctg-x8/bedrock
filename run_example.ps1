Param([parameter(mandatory = $true)][String]$Name)

pwsh -Command "`$Env:RUSTFLAGS = `"-L$Env:VK_SDK_PATH/Lib`"; cd bedrock-examples; cargo run --example $Name --features Presentation,VK_KHR_win32_surface,Allow1_3APIs"
