# Pre-Launch Prefix Feature Test

## What was implemented:

1. **Configuration Storage**: Added `pre_launch_prefix: Option<Vec<String>>` to `LauncherConfig`
2. **UI Components**: Added global pre-launch prefix dialog similar to global Java args
3. **Message Handling**: Added message variants for add/edit/delete/shift operations
4. **Launch Integration**: Modified the game launch command to prepend prefix commands

## How it works:

### Example Usage:

- Without prefix: `java -jar minecraft.jar`
- With prefix `["prime-run"]`: `prime-run java -jar minecraft.jar`
- With multiple prefixes `["prime-run", "--flag"]`: `prime-run --flag java -jar minecraft.jar`

### UI Location:

- Navigate to Settings → Internal tab
- Look for "Global Pre-Launch Prefix:" section (after Global Java Arguments)
- Add/edit/delete/reorder commands just like Java arguments

### Code Changes Summary:

1. **quantum_launcher/src/config.rs**: Added `pre_launch_prefix` field
2. **quantum_launcher/src/state/message.rs**: Added message handling variants
3. **quantum_launcher/src/menu_renderer/settings.rs**: Added UI integration
4. **quantum_launcher/src/menu_renderer/edit_instance.rs**: Added dialog function
5. **quantum_launcher/src/message_update/mod.rs**: Added message handlers
6. **quantum_launcher/src/message_handler/mod.rs**: Pass prefix to launch function
7. **quantum_launcher/src/cli/command.rs**: Updated CLI launch call
8. **crates/ql_instances/src/instance/launch/mod.rs**: Updated function signature
9. **crates/ql_instances/src/instance/launch/launcher.rs**:
   - Added prefix to struct
   - Modified command construction to prepend prefix

## Test Instructions:

1. Run the launcher: `cargo run`
2. Go to Settings → Internal
3. Find "Global Pre-Launch Prefix:" section
4. Add a test command like `echo` or `prime-run` (if available)
5. Try launching a game instance
6. Verify that the prefix command is prepended to the Java execution

## Expected Behavior:

- The prefix commands should be prepended to the final launch command
- Empty or whitespace-only prefix commands should be filtered out
- The feature should work similarly to the existing global Java arguments system
