$ErrorActionPreference = "Stop"
$commit_message = $args[0];
if ($commit_message -eq "") {
    Exit;
}
git add *;
git commit -m '"' + $commit_message + '"';
git push;