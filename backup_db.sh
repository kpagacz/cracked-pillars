git add ./hammer/hammer.db3 --verbose
git commit --allow-empty -m "Backup DB at $(date -I)"
git push --verbose origin deployment
