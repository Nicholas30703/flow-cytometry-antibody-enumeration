import init, { get_panels, enumerate_antibodies } from "./pkg/flow_cytometry_antibody_enumeration.js";

async function main() {
    await init();

    const panels = get_panels();
    const panelsContainer = document.getElementById("panels");
    const panelCheckboxes = [];

    for (const panel of panels) {
        const row = document.createElement("label");
        row.className = "panel";

        const checkbox = document.createElement("input");
        checkbox.type = "checkbox";
        checkbox.className = "panel-checkbox";
        checkbox.dataset.antibodies = JSON.stringify(panel.antibodies);
        checkbox.addEventListener("change", runEnumeration);
        panelCheckboxes.push(checkbox);

        const name = document.createElement("span");
        name.className = "panel-name";
        name.textContent = panel.name;

        const abs = document.createElement("span");
        abs.className = "panel-antibodies";
        abs.textContent = panel.antibodies.join(", ");

        row.appendChild(checkbox);
        row.appendChild(name);
        row.appendChild(abs);
        panelsContainer.appendChild(row);
    }

    function runEnumeration() {
        const selected = [];
        for (const cb of panelCheckboxes) {
            if (cb.checked) {
                selected.push(...JSON.parse(cb.dataset.antibodies));
            }
        }

        if (selected.length === 0) {
            document.getElementById("results").style.display = "none";
            return;
        }

        const result = enumerate_antibodies(JSON.stringify(selected));

        document.getElementById("results").style.display = "block";
        document.getElementById("count-badge").textContent = `${result.count} billable antibodies`;

        const listEl = document.getElementById("antibody-list");
        listEl.innerHTML = "";
        currentList = result.unique;
        for (const ab of result.unique) {
            const tag = document.createElement("span");
            tag.className = "antibody-tag";
            tag.textContent = ab;
            listEl.appendChild(tag);
        }
    }

    let currentList = [];

    document.getElementById("copy-btn").addEventListener("click", async () => {
        if (currentList.length === 0) return;
        let text;
        if (currentList.length === 1) {
            text = currentList[0];
        } else if (currentList.length === 2) {
            text = `${currentList[0]} and ${currentList[1]}`;
        } else {
            text = currentList.slice(0, -1).join(", ") + ", and " + currentList[currentList.length - 1];
        }
        await navigator.clipboard.writeText(text);
        const btn = document.getElementById("copy-btn");
        const original = btn.textContent;
        btn.textContent = "Copied!";
        setTimeout(() => { btn.textContent = original; }, 1500);
    });
}

main();
