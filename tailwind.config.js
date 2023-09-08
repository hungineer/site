module.exports = {
  purge: {
      mode: "all",
      content: [
          "./src/**/*.rs",
          "./index.html",
          "./src/**/*.html",
          "./src/**/*.css",
      ],
  },
  theme: {
    screens: {
      sm: '480px',
      md: '768px',
      lg: '976px',
      xl: '1440px',
    },
    extend: {
      colors: {
      	'blueee': '#1fb6ff',
      },
    },
  },
  variants: {},
  plugins: [],
};
