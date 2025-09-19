# UI Components Architecture

This document describes the refactored UI component architecture that provides a centralized theming system and reusable components.

## Directory Structure

```
ui/
├── components/
│   ├── common/           # Reusable components and theming
│   │   ├── colors.slint  # Central color palette and design tokens
│   │   ├── button.slint  # Reusable button components
│   │   ├── slider.slint  # Reusable slider components
│   │   └── mod.slint     # Export module for easy imports
│   └── player/           # Player-specific components
│       ├── main_controls.slint
│       ├── additional_controls.slint
│       ├── close_button.slint
│       ├── progress_bar.slint
│       ├── volume_slider.slint
│       └── ...
├── player.slint          # Main player window
└── main.slint            # Application entry point
```

## Color System (`components/common/colors.slint`)

### Global Color Palette

The centralized color system provides consistent theming across all components:

- **Background Colors**: Primary, secondary, and surface backgrounds
- **Text Colors**: Primary, secondary, and muted text
- **Button States**: Default, hover, and pressed states for different button types
- **Icon Colors**: Primary, secondary, and muted icon colors
- **Slider Colors**: Track, fill, and handle colors
- **Semantic Colors**: Success, warning, error, and info

### Design Tokens

- **Spacing**: Consistent spacing values (xs: 4px, sm: 8px, md: 12px, lg: 16px, xl: 24px, xxl: 32px)
- **BorderRadius**: Common border radius values (sm: 4px, md: 8px, lg: 12px, xl: 16px, full: 999px)
- **Animations**: Standard animation durations (fast: 100ms, normal: 200ms, slow: 300ms)

### Usage Example

```slint
import { Colors, Spacing, BorderRadius } from "../common/colors.slint";

Rectangle {
    background: Colors.background-primary;
    border-radius: BorderRadius.md;
    width: Spacing.xl;
}
```

## Button Components (`components/common/button.slint`)

### Available Components

1. **PrimaryButton**: Filled button for main actions (configurable shape)
2. **IconButton**: Transparent background button for secondary actions (configurable shape)
3. **CloseButton**: Pre-configured close button with X icon (circular)

### Button Sizes

- **Small**: 32px (used for compact UI elements)
- **Medium**: 52px (default size for most buttons)
- **Large**: 72px (used for primary actions like play/pause)

### Button Shapes

- **Circle**: Fully circular buttons (ideal for media controls)
- **Rounded-Square**: Rounded rectangle buttons (ideal for utility controls)

### Usage Examples

```slint
import { PrimaryButton, IconButton, ButtonSize, ButtonShape } from "../common/button.slint";

// Primary action button (circular for media controls)
PrimaryButton {
    size: ButtonSize.large;
    shape: ButtonShape.circle;
    clicked => { /* handle click */ }
    
    Image {
        source: @image-url("icons/play.svg");
        width: 32px;
        height: 32px;
        colorize: Colors.icon-primary;
    }
}

// Secondary action button (rounded square for utility controls)
IconButton {
    size: ButtonSize.medium;
    shape: ButtonShape.rounded-square;
    clicked => { /* handle click */ }
    
    Image {
        source: @image-url("icons/settings.svg");
        width: 24px;
        height: 24px;
        colorize: Colors.icon-secondary;
    }
}

// Navigation button (circular for media controls)
IconButton {
    size: ButtonSize.medium;
    shape: ButtonShape.circle;
    clicked => { /* handle click */ }
    
    Image {
        source: @image-url("icons/skip.svg");
        width: 24px;
        height: 24px;
        colorize: Colors.icon-secondary;
    }
}
```

## Slider Components (`components/common/slider.slint`)

### Available Components

1. **Slider**: Base slider component for interactive controls
2. **ProgressBar**: Progress display with time labels
3. **VolumeSlider**: Volume control with speaker icons

### Features

- Smooth animations and transitions
- Touch/mouse interaction support
- Customizable appearance (track height, handle size)
- Accessibility-friendly design

### Usage Examples

```slint
import { ProgressBar, VolumeSlider } from "../common/slider.slint";

// Progress bar for music playback
ProgressBar {
    progress: 0.25;
    current-time: "1:30";
    remaining-time: "-2:45";
    
    seek(value) => {
        // Handle seek operation
    }
}

// Volume control
VolumeSlider {
    volume: 0.7;
    
    volume-changed(value) => {
        // Handle volume change
    }
}
```

## Migration Benefits

### Before Refactoring

- Hardcoded colors scattered throughout components
- Duplicated button logic and styling
- Inconsistent spacing and animations
- Difficult to maintain consistent theming

### After Refactoring

- ✅ Centralized color management
- ✅ Consistent component behavior
- ✅ Easy theme customization
- ✅ Reduced code duplication
- ✅ Standardized animations and interactions
- ✅ Type-safe component properties

## Customizing Colors

To change the application's color scheme, modify the values in `components/common/colors.slint`:

```slint
export global Colors {
    // Change primary background
    out property <brush> background-primary: #2a1f2d; // Dark purple theme
    
    // Change accent colors
    out property <brush> accent-primary: #8b5cf6;     // Purple accent
    out property <brush> accent-secondary: #06d6a0;   // Green accent
    
    // All other colors remain consistent
}
```

## Import Patterns

### Individual Imports
```slint
import { Colors, Spacing } from "../common/colors.slint";
import { IconButton, ButtonSize } from "../common/button.slint";
```

### Consolidated Imports
```slint
import { 
    Colors, 
    Spacing, 
    IconButton, 
    ButtonSize, 
    VolumeSlider 
} from "../common/mod.slint";
```

## Best Practices

1. **Always use the color system** instead of hardcoded hex values
2. **Use design tokens** for consistent spacing and sizing
3. **Leverage reusable components** instead of creating custom buttons/sliders
4. **Import through mod.slint** for cleaner import statements
5. **Follow the established naming conventions** for consistency

This architecture provides a solid foundation for maintainable, themeable UI components that follow Slint best practices.