const path = require('path');

module.exports = {
    entry: './components.js',
    output: {
        path: path.resolve(__dirname, ''),
        filename: 'bundle.js',
    },
};