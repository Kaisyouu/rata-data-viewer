# Example: Testing the Viewer

## Quick Test Command

To test with your CTPDepthMarketData.parquet file:

```bash
cd /home/huaisy/github/rata-data-viewer
./target/release/rata-data-viewer /home/huaisy/data/CTPDepthMarketData.parquet
```

## What to Test

### Basic Navigation
1. **Verify data loads**: You should see a table with your market data
2. **Scroll vertically**: Use `↓`/`↑` or `j`/`k` to scroll through rows
3. **Scroll horizontally**: Use `→`/`←` or `l`/`h` to see more columns
4. **Jump navigation**: Press `g` to go to top, `G` to go to bottom

### Filtering
1. **Enter filter mode**: Press `/`
2. **Search for an instrument**: Type part of an instrument code (e.g., "000001")
3. **Apply filter**: Press `Enter`
4. **Clear filter**: Press `Esc`

### Page Navigation
1. **Page down**: Press `PgDn` to skip 20 rows
2. **Page up**: Press `PgUp` to go back 20 rows

### Exit
- Press `q` or `Ctrl+C` to quit

## Expected Behavior

When viewing the CTPDepthMarketData.parquet file, you should see:
- **Header**: File path and keyboard shortcuts
- **Table**: Market data with columns like instrument code, price, volume, etc.
- **Footer**: Shows total rows and visible column range
- **Status bar**: Shows "Ready" in green

## If Something Goes Wrong

### Terminal garbled after crash
```bash
reset
```

### Can't see colors
- Make sure you're using a modern terminal (not old versions of cmd.exe)
- Try: Windows Terminal, iTerm2, Alacritty, or gnome-terminal

### Data doesn't load
- Check file path is correct
- Verify file is valid Parquet format:
  ```bash
  file /home/huaisy/data/CTPDepthMarketData.parquet
  ```

## Feedback Areas

Please test and provide feedback on:
1. **UI Layout**: Is everything readable? Too cramped?
2. **Navigation**: Is scrolling smooth? Any lag?
3. **Filtering**: Does search work as expected? Too slow?
4. **Column display**: Can you see enough columns? Need more?
5. **Performance**: How long does it take to load? Any delays when filtering?
6. **Missing features**: What would you like to see added?

## Sample Test Session

```
# Start the viewer
./target/release/rata-data-viewer /home/huaisy/data/CTPDepthMarketData.parquet

# What you'll do:
1. Look at the data - does it look correct?
2. Press 'j' multiple times - does it scroll smoothly?
3. Press 'l' - do you see more columns?
4. Press '/' and type "0000" - does it find relevant rows?
5. Press 'G' - does it jump to the end?
6. Press 'g' - does it jump back to the start?
7. Press 'q' - does it exit cleanly?
```
