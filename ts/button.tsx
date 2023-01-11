import * as React from 'react';
import * as ReactDOM from 'react-dom';

function App() {
    return (
        <>
            <h1>Colorful Custom Button Components</h1>
        </>
    )
}

function render_app() {
    const root = ReactDOM.createRoot(document.getElementById("react-root"));
    root.render(<App />);
}

export default render_app;