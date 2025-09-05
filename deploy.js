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
    // 1. Build the project (Rust + Tailwind)
    run("dx build --release");
    run("npx tailwindcss -i ./assets/tailwind.css -o ./target/dx/t4g/release/web/public/tailwind.css --minify");

    // 2. Copy build to temporary folder
    fs.removeSync(TMP_DIR);
    fs.copySync(OUTPUT, TMP_DIR);

    // 3. Switch to gh-pages branch
    run("git checkout gh-pages || git checkout -b gh-pages");

    // 4. Clear old files but keep .git
    fs.readdirSync(__dirname).forEach(f => {
        if (f !== ".git") fs.removeSync(path.join(__dirname, f));
    });

    // 5. Copy new build from temp
    fs.copySync(TMP_DIR, __dirname);

    // 6. Commit & push
    run("git add .");
    try {
        run(`git commit -m "Deploy ${new Date().toISOString()}"`);
    } catch {
        console.log("ℹ️ No changes to commit");
    }
    run("git push origin gh-pages --force");

    // 7. Clean up temp
    fs.removeSync(TMP_DIR);

    // 8. Switch back to main
    run("git checkout main");
}

main();
