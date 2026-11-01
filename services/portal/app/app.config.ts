// Role of each brand palette (defined in `assets/css/main.css`).
export default defineAppConfig({
  ui: {
    colors: {
      primary: "navy",
      secondary: "teal",
      tertiary: "sun",
      success: "teal",
      info: "navy",
      warning: "rose",
      error: "rose",
      neutral: "sand",
    },
    // The theme radius would make checkboxes round, like radio buttons.
    checkbox: {
      slots: {
        base: "rounded-xs",
      },
    },
    formField: {
      slots: {
        label: "text-xs font-semibold tracking-wider text-muted uppercase",
      },
    },
  },
});
