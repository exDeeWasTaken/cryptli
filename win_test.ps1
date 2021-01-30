Invoke-Expression "cd deno";

Invoke-Expression "deno test --allow-read";

Invoke-Expression "cd ../node";

Invoke-Expression "npm test";

Invoke-Expression "cd ../";