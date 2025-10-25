# IBM 1130 Punch Card Simulator - Implementation Documentation

## Overview

This document tracks the development of an interactive IBM 1130 punch card simulator built with Yew (Rust/WASM), served via a CLI application. The simulator recreates the experience of punching cards on an IBM 029 keypunch machine, supporting both text input (with Hollerith encoding) and binary data (for object decks).

---

## Requirements

### Functional Requirements

#### Core Functionality
1. **Web Application**: Yew-based SPA that renders an SVG punch card
2. **CLI Server**: Rust CLI tool to serve the app on a configurable port
   - Default port: 9267
   - Command-line options: `-p/--port <PORT>`
3. **Punch Card Visualization**: SVG-based representation of an 80-column IBM punch card
   - 80 columns (numbered 1-80)
   - 12 rows per column (12, 11, 0-9)
   - Starts blank (no punches)
   - Punches appear as black rectangles

#### Input Modes

**Text Mode**
- Character-by-character input (not line-by-line)
- Converts text to Hollerith encoding
- Highlights current column being punched
- Prints characters at top of card (IBM 029 feature)
- Supports IBM 1130 assembler source code format

**Binary Mode**
- Upload 80-byte binary files
- Represents IBM 1130 object deck format
- Uses columns 1-72, all 12 rows (864 bits)
- No character printing at top for binary cards

#### Example Generators
- Generate example IBM 1130 assembler source card
- Generate example IBM 1130 object deck card
- Allow users to load examples for learning

### Non-Functional Requirements

1. **Code Quality**
   - Modular, reusable Yew components
   - Test-Driven Development (Red/Green)
   - Comprehensive unit tests
   - Well-documented code

2. **Documentation**
   - Comprehensive README.md
   - Implementation documentation (this file)
   - Architecture diagrams
   - Usage examples

3. **Testing**
   - Unit tests for all modules
   - Integration tests for components
   - Playwright visual/UI tests
   - Screenshot capture for documentation

4. **Deployment**
   - GitHub Actions workflow
   - GitHub Pages hosting
   - Live demo link in README

---

## Architecture

### System Overview

```
┌─────────────────────────────────────────────────────────────┐
│                     CLI Application (Rust)                   │
│  ┌────────────────────────────────────────────────────────┐ │
│  │  Command Line Parser (clap)                            │ │
│  │  - Parse port argument (-p/--port)                     │ │
│  │  - Default: 9267                                       │ │
│  └────────────────────────────────────────────────────────┘ │
│  ┌────────────────────────────────────────────────────────┐ │
│  │  HTTP Server (warp or actix-web)                       │ │
│  │  - Serve static WASM bundle                            │ │
│  │  - Serve HTML/CSS/JS assets                            │ │
│  └────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
                           │
                           │ HTTP
                           ▼
┌─────────────────────────────────────────────────────────────┐
│              Yew Web Application (WASM)                      │
│  ┌────────────────────────────────────────────────────────┐ │
│  │  App Component                                         │ │
│  │  - State management                                    │ │
│  │  - Mode switching (text/binary)                        │ │
│  │  - Example loading                                     │ │
│  └────────────────────────────────────────────────────────┘ │
│  ┌────────────────────────────────────────────────────────┐ │
│  │  PunchCard Component (SVG)                             │ │
│  │  - Renders 80x12 grid                                  │ │
│  │  - Displays punches                                    │ │
│  │  - Column highlighting                                 │ │
│  │  - Character printing (top)                            │ │
│  └────────────────────────────────────────────────────────┘ │
│  ┌────────────────────────────────────────────────────────┐ │
│  │  TextInput Component                                   │ │
│  │  - Character-by-character capture                      │ │
│  │  - Current position tracking                           │ │
│  │  - Clear/Reset functionality                           │ │
│  └────────────────────────────────────────────────────────┘ │
│  ┌────────────────────────────────────────────────────────┐ │
│  │  FileUpload Component                                  │ │
│  │  - Accept 80-byte binary files                         │ │
│  │  - Validation                                           │ │
│  │  - Error handling                                      │ │
│  └────────────────────────────────────────────────────────┘ │
│  ┌────────────────────────────────────────────────────────┐ │
│  │  ExampleGenerator Component                            │ │
│  │  - Generate IBM 1130 source example                    │ │
│  │  - Generate IBM 1130 object deck example               │ │
│  │  - Load examples into card                             │ │
│  └────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
                           │
                           │ Uses
                           ▼
┌─────────────────────────────────────────────────────────────┐
│                   Core Modules (Rust)                        │
│  ┌────────────────────────────────────────────────────────┐ │
│  │  hollerith_encoding                                    │ │
│  │  - Character to punch pattern mapping                  │ │
│  │  - Punch pattern to character mapping                  │ │
│  │  - Support for 64-character set (IBM 029)              │ │
│  └────────────────────────────────────────────────────────┘ │
│  ┌────────────────────────────────────────────────────────┐ │
│  │  punch_card                                            │ │
│  │  - Data structure for 80 columns                       │ │
│  │  - Punch operations                                    │ │
│  │  - Serialization/deserialization                       │ │
│  └────────────────────────────────────────────────────────┘ │
│  ┌────────────────────────────────────────────────────────┐ │
│  │  ibm1130_format                                        │ │
│  │  - Assembler source format                             │ │
│  │  - Object deck format (binary)                         │ │
│  │  - Example generation                                  │ │
│  └────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

### Component Hierarchy

```
App
├── ControlPanel
│   ├── ModeSelector (Text/Binary)
│   └── ExampleLoader
├── InputArea
│   ├── TextInput (for text mode)
│   └── FileUpload (for binary mode)
└── CardDisplay
    ├── PunchCard (SVG)
    │   ├── CardBackground
    │   ├── ColumnNumbers
    │   ├── PrintedCharacters
    │   ├── PunchGrid
    │   │   └── Punch (x80 columns, x12 rows)
    │   └── ColumnHighlight
    └── CardInfo
        ├── CurrentColumn
        └── CardType (Source/Binary)
```

### Technology Stack

- **Rust Edition**: 2024
- **Frontend Framework**: Yew (latest)
- **WASM Build Tool**: Trunk
- **CLI Framework**: clap (v4)
- **HTTP Server**: warp or actix-web
- **Testing Framework**:
  - `cargo test` for Rust unit tests
  - `wasm-bindgen-test` for WASM tests
  - Playwright (via MCP) for UI tests
- **CI/CD**: GitHub Actions
- **Deployment**: GitHub Pages

---

## Design

### Data Structures

#### PunchCard
```rust
pub struct PunchCard {
    columns: [Column; 80],
    card_type: CardType,
}

pub struct Column {
    punches: [bool; 12],  // Rows 12, 11, 0-9
    printed_char: Option<char>,
}

pub enum CardType {
    Text,
    Binary,
}
```

#### Hollerith Encoding
```rust
pub struct HollerithCode {
    rows: Vec<u8>,  // Row numbers that are punched (12, 11, 0-9)
}

pub fn char_to_hollerith(c: char) -> Option<HollerithCode>;
pub fn hollerith_to_char(code: &HollerithCode) -> Option<char>;
```

### Hollerith Encoding Table

Based on IBM 029 keypunch (64 printable characters):

| Char | Rows       | Char | Rows       | Char | Rows       |
|------|------------|------|------------|------|------------|
| 0-9  | 0-9        | A-I  | 12,1-9     | J-R  | 11,1-9     |
| S-Z  | 0,2-9      | blank| (none)     | .    | 12,3,8     |
| <    | 12,4,8     | (    | 12,5,8     | +    | 12,6,8     |
| \|   | 12,7,8     | &    | 12         | !    | 11,2,8     |
| $    | 11,3,8     | *    | 11,4,8     | )    | 11,5,8     |
| ;    | 11,6,8     | ¬    | 11,7,8     | -    | 11         |
| /    | 0,1        | ,    | 0,3,8      | %    | 0,4,8      |
| _    | 0,5,8      | >    | 0,6,8      | ?    | 0,7,8      |
| :    | 2,8        | #    | 3,8        | @    | 4,8        |
| '    | 5,8        | =    | 6,8        | "    | 7,8        |

### IBM 1130 Specific Formats

#### Assembler Source Card
- Column 1-5: Label (optional)
- Column 6-10: Blank or continuation
- Column 11-15: Opcode
- Column 16-80: Operands and comments
- Characters printed at top

#### Object Deck Card (Binary)
- Columns 1-72: Binary data (all 12 rows used)
- Columns 73-80: Sequence/identification (optional)
- No character printing
- 4:3 format: 4 Hollerith words → 3 binary 16-bit words

### UI/UX Design

#### Layout
```
┌─────────────────────────────────────────────────────────────┐
│  IBM 1130 Punch Card Simulator                              │
├─────────────────────────────────────────────────────────────┤
│  Mode: ○ Text  ○ Binary    Examples: [Source] [Object]     │
├─────────────────────────────────────────────────────────────┤
│  Input:                                                      │
│  [Text input field] or [Choose File] button                 │
├─────────────────────────────────────────────────────────────┤
│  Punch Card:                                                 │
│  ┌───────────────────────────────────────────────────────┐  │
│  │  1  2  3  4  5 ... 78 79 80                           │  │
│  │  A  B  C  D  E ... (printed characters)               │  │
│  │ ┌─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┐                            │  │
│  │ │ │█│ │ │█│ │ │ │ │ │ │ │ 12 Row                    │  │
│  │ ├─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┤                            │  │
│  │ │█│ │ │█│ │ │ │ │ │ │ │ │ 11 Row                    │  │
│  │ ├─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┤                            │  │
│  │ │ │ │█│ │ │ │ │ │ │ │ │ │  0 Row                    │  │
│  │ ... (rows 1-9)                                        │  │
│  │  [Column 5 highlighted]                               │  │
│  └───────────────────────────────────────────────────────┘  │
│  Column: 5/80     Type: Text                                │
└─────────────────────────────────────────────────────────────┘
```

#### Visual Design
- Card background: Cream/manila color (#f4e8d0)
- Punches: Black rectangles
- Column highlight: Light blue overlay
- Printed text: Black, monospace font
- Responsive design for mobile/tablet/desktop

---

## Plan

### Phase 1: Project Setup
- [x] Research IBM 1130 formats and encoding
- [ ] Create project structure
- [ ] Setup Cargo workspace
- [ ] Configure Trunk for WASM builds
- [ ] Setup testing infrastructure

### Phase 2: Core Modules (TDD)
- [ ] Implement `hollerith_encoding` module
  - [ ] Write tests for character encoding
  - [ ] Implement encoding functions
  - [ ] Write tests for decoding
  - [ ] Implement decoding functions
- [ ] Implement `punch_card` module
  - [ ] Write tests for card structure
  - [ ] Implement card data structure
  - [ ] Write tests for punch operations
  - [ ] Implement punch operations
- [ ] Implement `ibm1130_format` module
  - [ ] Write tests for source format
  - [ ] Implement source format
  - [ ] Write tests for binary format
  - [ ] Implement binary format

### Phase 3: Yew Components (TDD)
- [ ] Create `PunchCard` component
  - [ ] Write component tests
  - [ ] Implement SVG rendering
  - [ ] Add column highlighting
  - [ ] Add character printing
- [ ] Create `TextInput` component
  - [ ] Write component tests
  - [ ] Implement character capture
  - [ ] Implement position tracking
- [ ] Create `FileUpload` component
  - [ ] Write component tests
  - [ ] Implement file validation
  - [ ] Implement binary loading
- [ ] Create `ExampleGenerator` component
  - [ ] Write component tests
  - [ ] Implement example generation
- [ ] Create `App` component
  - [ ] Write integration tests
  - [ ] Implement state management
  - [ ] Wire up all components

### Phase 4: CLI Server
- [ ] Implement CLI with clap
  - [ ] Add port argument parsing
  - [ ] Add help text
- [ ] Setup HTTP server (warp/actix-web)
  - [ ] Serve static files
  - [ ] Configure CORS if needed
- [ ] Build system integration
  - [ ] Trunk build integration
  - [ ] Asset bundling

### Phase 5: Testing & Documentation
- [ ] Write comprehensive unit tests
- [ ] Playwright UI tests
  - [ ] Test text input mode
  - [ ] Test binary upload mode
  - [ ] Test example generation
  - [ ] Capture screenshot
- [ ] Write README.md
  - [ ] Installation instructions
  - [ ] Usage examples
  - [ ] Screenshot
  - [ ] Link to live demo
- [ ] Complete implementation.md

### Phase 6: Deployment
- [ ] Setup GitHub Actions workflow
  - [ ] Build WASM bundle
  - [ ] Run tests
  - [ ] Deploy to GitHub Pages
- [ ] Configure GitHub Pages
- [ ] Add live demo link to README

---

## Status

### Completed
- ✅ Research IBM 1130 punch card formats
- ✅ Research Hollerith encoding (IBM 029)
- ✅ Research IBM 029 character printing feature
- ✅ Research IBM 1130 object deck binary format
- ✅ Created initial implementation documentation

### In Progress
- 🔄 Creating implementation documentation

### Not Started
- ⏳ Project structure setup
- ⏳ Core module implementation
- ⏳ Component development
- ⏳ CLI development
- ⏳ Testing
- ⏳ Documentation
- ⏳ Deployment

### Blockers
None currently.

---

## Technical Notes

### Hollerith Encoding Insights
- **Row numbering**: Rows are numbered 12 (top), 11, 0, 1-9 (bottom)
- **Zone punches**: Rows 12, 11, 0 are called "zone" rows
- **Numeric punches**: Rows 1-9 are "numeric" rows
- **Character encoding**: Most characters use zone + numeric combination
- **Notation**: Punches written as "12-4-8" means rows 12, 4, and 8

### IBM 029 Character Printing
- **Mechanism**: 5×7 dot matrix
- **Timing**: Prints as each column is punched
- **Purpose**: Human readability
- **Binary cards**: No printing for binary/object decks

### IBM 1130 Binary Cards
- **Data columns**: 1-72 (not full 80)
- **Bit count**: 864 bits (72 columns × 12 rows)
- **Format**: 4:3 conversion (4 Hollerith words → 3×16-bit words)
- **Usage**: Compiled object code, system software

### Development Considerations
1. **WASM limitations**: File system access restricted; use File API
2. **SVG performance**: Optimize for 960 punch positions (80×12)
3. **Mobile support**: Touch-friendly input, responsive layout
4. **Accessibility**: ARIA labels, keyboard navigation
5. **Browser compatibility**: Test on Chrome, Firefox, Safari, Edge

---

## References

### Documentation
- [IBM 029 Card Punch](https://twobithistory.org/2018/06/23/ibm-029-card-punch.html)
- [Doug Jones's Punched Card Codes](https://homepage.divms.uiowa.edu/~jones/cards/codes.html)
- [IBM 1130 Wikipedia](https://en.wikipedia.org/wiki/IBM_1130)
- [IBM 1130 Binary Card Format](https://dialectrix.com/G4G/ZebraStripeCard.html)

### Similar Projects
- Reference: toggle-nixie (React/Vite/SVG)
- Reference: knob-lamps (React/Vite/SVG)

### Tools & Frameworks
- [Yew Documentation](https://yew.rs/)
- [Trunk Documentation](https://trunkrs.dev/)
- [clap Documentation](https://docs.rs/clap/)
- [wasm-bindgen-test](https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/index.html)

---

*Last Updated: 2025-10-25*
