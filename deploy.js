const { execSync } = require("child_process");
const fs = require("fs-extra");
const path = require("path");

// Paths
const OUTPUT = path.join(__dirname, "target", "dx", "t4g", "release", "web", "public");
const TMP_DIR = path.join(__dirname, "_deploy_tmp");

function run(cmd) {
    execSync(cmd, { stdio: "inherit" });
}

function main() {
    // 1️⃣ Make sure we're on main branch
    const branch = execSync("git branch --show-current").toString().trim();
    if (branch !== "main") {
        console.error("❌ You must run this deploy script from the main branch!");
        process.exit(1);
    }

    // 2️⃣ Make sure working directory is clean
    const status = execSync("git status --porcelain").toString().trim();
    if (status) {
        console.error("❌ Please commit or stash your changes before deploying:\n", status);
        process.exit(1);
    }

    // 3️⃣ Build Rust + Tailwind
    run("dx build --release");
    run("npx tailwindcss -i ./assets/tailwind.css -o ./target/dx/t4g/release/web/public/tailwind.css --minify");

    // 4️⃣ Copy output to temporary folder
    fs.removeSync(TMP_DIR);
    fs.mkdirSync(TMP_DIR, { recursive: true });
    fs.copySync(OUTPUT, TMP_DIR);

    // 5️⃣ Switch to gh-pages
    run("git checkout gh-pages");

    // 6️⃣ Remove old files but keep .git
    fs.readdirSync(__dirname).forEach(f => {
        if (f !== ".git") fs.removeSync(path.join(__dirname, f));
    });

    // 7️⃣ Copy new build from temp
    fs.copySync(TMP_DIR, __dirname);

    // 8️⃣ Commit & push
    run("git add .");
    try {
        run(`git commit -m "Deploy ${new Date().toISOString()}"`);
    } catch {
        console.log("ℹ️ No changes to commit");
    }
    run("git push origin gh-pages --force");

    // 9️⃣ Clean up temp
    fs.removeSync(TMP_DIR);

    // 🔟 Switch back to main
    run("git checkout main");

    console.log("✅ Deployment complete!");
}

main();
