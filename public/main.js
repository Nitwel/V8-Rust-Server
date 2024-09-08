const button = document.getElementById("run")
const input = document.getElementById("code_area")
const output = document.getElementById("result")
const logs = document.getElementById("console")
const username = document.getElementById("username")
const password = document.getElementById("password")
const login = document.getElementById("login")
const auth = document.getElementById("auth")

// Snippets
const save_snippet = document.getElementById("create_snippet")
const remove_snippet = document.getElementById("delete_snippet")
const snippet_name = document.getElementById("snippet_name")
const snippets_select = document.getElementById("snippets")

let token = null;
let snippets = [];
let current_snippet = null;

save_snippet.addEventListener("click", async () => {
    if (!snippet_name.value) {
        alert("Please enter snippet name")
        return;
    }

    if (!input.value) {
        alert("Please enter snippet code")
        return;
    }

    if (current_snippet) {
        const result = await fetch(`/api/snippets/${current_snippet}`, {
            method: "PUT",
            body: JSON.stringify({
                id: current_snippet,
                title: snippet_name.value,
                body: input.value
            }),
            headers: {
                "Content-Type": "application/json",
                Authorization: token
            }
        })

        if (result.ok) {
            alert("Snippet updated successfully")
            load_snippets();
        } else {
            alert("Failed to update snippet")
        }

        return;
    }

    const result = await fetch("/api/snippets", {
        method: "POST",
        body: JSON.stringify({
            title: snippet_name.value,
            body: input.value
        }),
        headers: {
            "Content-Type": "application/json",
            Authorization: token
        }
    })

    if (result.ok) {
        alert("Snippet created successfully")
        load_snippets();
    } else {
        alert("Failed to create snippet")
    }
})

remove_snippet.addEventListener("click", async () => {
    if (!current_snippet) {
        alert("Please select a snippet")
        return;
    }

    const result = await fetch(`/api/snippets/${current_snippet}`, {
        method: "DELETE",
        headers: {
            Authorization: token
        }
    })

    if (result.ok) {
        alert("Snippet deleted successfully")
        load_snippets();
    }
})

snippets_select.addEventListener("change", async () => {
    current_snippet = snippets_select.value;

    if (current_snippet === "0") {
        current_snippet = null;
        snippet_name.value = "";
        input.value = "";
        return;
    }

    const snippet = snippets.find(s => s.id == current_snippet);

    if (!snippet) {
        alert("Snippet not found")
        return;
    }

    snippet_name.value = snippet.title;
    input.value = snippet.body;
})

button.addEventListener("click", async () => {
    const result = await fetch("/api/run", {
        method: "POST",
        body: input.value
    })

    const data = await result.json();

    output.value = data.result;
    logs.value = data.logs.join("\n");
})

login.addEventListener("click", async () => {
    if (!username.value || !password.value) {
        alert("Please enter username and password")
        return;
    }

    if (token) {
        const result = await fetch("/api/logout", {
            method: "POST",
            headers: {
                Authorization: token
            }
        })

        if (result.ok) {
            token = null;
            auth.classList.remove("success")
            login.innerText = "Login"
            snippets = [];
            snippets_select.innerHTML = "";
            alert("Logged out successfully")
        }
    } else {
        const result = await fetch("/api/login", {
            method: "POST",
            body: JSON.stringify({
                username: username.value,
                password: password.value
            }),
            headers: {
                "Content-Type": "application/json"
            }
        })

        if (result.ok) {
            const data = await result.json();
            token = data.token;
            auth.classList.add("success")
            login.innerText = "Logout"
            load_snippets();
            alert("Logged in successfully")
        } else {
            alert("Invalid username or password")
        }
    }
})

async function load_snippets() {
    const result = await fetch("/api/snippets", {
        headers: {
            Authorization: token
        }
    })

    snippets = await result.json();

    snippets_select.innerHTML = "";

    const option = document.createElement("option");
    option.value = "0";
    option.innerText = "Select snippet";
    snippets_select.appendChild(option);

    snippets.forEach(snippet => {
        const option = document.createElement("option");
        option.value = snippet.id;
        option.innerText = snippet.title;
        snippets_select.appendChild(option);
    })
}