 Param
 (
 [parameter(Mandatory=$true)]
 [ValidateNotNull()]
 $commit_message
 )

git add *;
git commit -m $commit_message;
git push;