const { execSync } = require("child_process");
const fs = require("fs-extra");
const path = require("path");

// Utility to run commands
function run(cmd) {
    execSync(cmd, { stdio: "inherit" });
}

// Paths
const mainBranch = "main";
const ghPagesBranch = "gh-pages";
const buildDir = path.join("target", "dx", "t4g", "release", "web", "public");
const tmpDir = "_deploy_tmp";

function main() {
    try {
        // Build the project
        console.log("🚀 Building project...");
        run("dx build --release");

        // Make sure the build folder exists
        if (!fs.existsSync(buildDir)) {
            console.error(`❌ Build folder does not exist: ${buildDir}`);
            process.exit(1);
        }

        // Stash any tracked changes (like Cargo.toml CRLF changes)
        console.log("💾 Stashing tracked changes...");
        run("git stash push -k -u || echo 'No tracked changes to stash'");

        // Switch to gh-pages branch (create if doesn't exist)
        try {
            run(`git checkout ${ghPagesBranch}`);
        } catch {
            run(`git checkout -b ${ghPagesBranch}`);
        }

        // Clear old contents except .git
        fs.readdirSync(".")
            .filter(f => f !== ".git")
            .forEach(f => fs.rmSync(f, { recursive: true, force: true }));

        // Copy build output to root
        console.log(`📂 Copying build files from ${buildDir}...`);
        fs.copySync(buildDir, ".", { overwrite: true });

        // Commit and push
        run('git add .');
        run('git commit -m "Deploy site" || echo "No changes to commit"');
        run(`git push origin ${ghPagesBranch}`);

        // Switch back to main and pop stash
        run(`git checkout ${mainBranch}`);
        run("git stash pop || echo 'No stashed changes to pop'");

        console.log("✅ Deployment complete!");
    } catch (err) {
        console.error("❌ Deployment failed:", err);
        process.exit(1);
    }
}

main();
