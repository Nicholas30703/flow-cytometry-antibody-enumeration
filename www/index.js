import init, { get_panels, enumerate_antibodies } from "./pkg/flow_cytometry_antibody_enumeration.js";

async function main() {
    await init();

    const panels = get_panels();
    const panelsContainer = document.getElementById("panels");
    const panelCheckboxes = [];

    for (const panel of panels) {
        const div = document.createElement("div");
        div.className = "panel";

        const header = document.createElement("label");
        header.className = "panel-header";

        const checkbox = document.createElement("input");
        checkbox.type = "checkbox";
        checkbox.dataset.antibodies = JSON.stringify(panel.antibodies);
        checkbox.addEventListener("change", runEnumeration);
        panelCheckboxes.push(checkbox);

        const h3 = document.createElement("h3");
        h3.textContent = panel.name;

        header.appendChild(checkbox);
        header.appendChild(h3);
        div.appendChild(header);

        for (const ab of panel.antibodies) {
            const item = document.createElement("div");
            item.className = "antibody-item";
            item.textContent = ab;
            div.appendChild(item);
        }

        panelsContainer.appendChild(div);
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
