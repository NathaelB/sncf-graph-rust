echo "Configuration des serveurs de configuration MongoDB..."


initConfigServers() {
  docker exec config1 mongo --eval 'rs.initiate({_id: "configReplSet", configsvr: true, members: [{ _id : 0, host : "config1" },{ _id : 1, host : "config2" },{ _id : 2, host : "config3" }]})'
}

echo "Initialisation des shards..."

initShard() {
  local containerName="$1"
  local replSetName="$2"

  docker exec "$containerName" mongo --eval "rs.initiate({_id: \"$replSetName\", members: [{ _id: 0, host: \"$containerName\" }]})"
}

# Ajouter les shards au cluster via mongos
addShards() {
    docker exec mongos mongo --eval 'sh.addShard("shard1ReplSet/shard1-1:27017"); sh.addShard("shard2ReplSet/shard2-1:27017"); sh.addShard("shard3ReplSet/shard3-1:27017");'
}

initConfigServers


initShard "shard1-1" "shard1ReplSet"
initShard "shard2-1" "shard2ReplSet"
initShard "shard3-1" "shard3ReplSet"
addShards

echo "Configuration du cluster MongoDB terminée."