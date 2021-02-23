cd deno || exit;

deno test --allow-read;

cd ../node || exit;

npm test;

cd ../;