import React, { useState } from 'react';
import { createRoot } from 'react-dom/client';

function Counter() {
    const [count, setCount] = useState(0);
    return (
        <>
            <h1>{count}</h1>
            <button onClick={() => setCount(count + 1)}>
                Increment
            </button>
        </>
    );
}

export function render_counter() {
    let el = document.getElementById('root');
    if (el != null) {
        const root = createRoot(el);
        root.render(<Counter />);
    } else {
        console.log("Failed to get element root");
    }
}
