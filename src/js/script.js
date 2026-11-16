const { listen } = window.__TAURI__.event;
const { open } = window.__TAURI__.dialog;
const { invoke } = window.__TAURI__.core;
const stus = document.getElementById("status");

async function runExtraction(filePath) {
    if (!filePath) return;

    const outDir = await open({
        directory: true,
        multiple: false,
        title: "Choose an output folder"
    });

    if (!outDir) return;

    stus.textContent = "Extracting...";

    try {
        const msg = await invoke("extract_assets", { filePath, outDir });
        stus.textContent = msg;
    } catch (err) {
        stus.textContent = `${err}`;
    }
}

listen("tauri://drag-drop", async (event) => {
    const filePath = event.payload.paths?.[0];
    await runExtraction(filePath);
});

async function handleDropzoneClick() {
    const filePath = await open({
        multiple: false,
        directory: false,
        title: "Select a Tauri .exe to extract",
        filters: [{ name: "Executable", extensions: ["exe"] }]
    });

    await runExtraction(filePath);
}

const blockScroll = (e) => {
    e.preventDefault();
};

window.addEventListener("wheel", blockScroll, { passive: false });
window.addEventListener("touchmove", blockScroll, { passive: false });

document.addEventListener("contextmenu", (event) => {
    event.preventDefault();
});

window.addEventListener("keydown", (event) => {
    if (/^F\d+$/.test(event.key)) {
        event.preventDefault();
    }
});