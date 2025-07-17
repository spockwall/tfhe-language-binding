const path = require("path");

module.exports = {
    entry: { tfhe: "./src/tfhe.js", custom: "./src/custom.js" },

    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "[name].js",
        //library: "NewPackage",
        //libraryTarget: "umd",
    },
    module: {
        rules: [
            {
                test: /\.js$/,
                exclude: /node_modules/,
                use: {
                    loader: "babel-loader",
                    options: {
                        presets: ["@babel/preset-env"],
                    },
                },
            },
        ],
    },
    mode: "development",
};
