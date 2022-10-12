import path from "path";

module.exports = {
    build: {
        emptyOutDir: false,
        lib: {
            entry: path.resolve(__dirname, "src/embed.ts"),
            name: "Embed",
            fileName: (format) => `embed.${format}.js`,
        },
    },
};
