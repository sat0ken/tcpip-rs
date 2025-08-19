#!/bin/bash

# rootユーザーが必要
if [ $UID -ne 0 ]; then
  echo "Root privileges are required"
  exit 1;
fi

# 全てのnetnsを削除
ip -all netns delete

# 2つのnetnsを作成
ip netns add host1
ip netns add host2

# host1とhost2のリンクの作成
ip link add name host1-host2 type veth peer name host2-host1

# リンクの割り当て
ip link set host1-host2 netns host1
ip link set host2-host1 netns host2

# host1のリンクの設定
ip netns exec host1 ip addr add 2001:db8:0:1::2/64 dev host1-host2
ip netns exec host1 ip link set host1-host2 up
ip netns exec host1 ethtool -K host1-host2 rx off tx off
ip netns exec host1 ip route add default via 2001:db8:0:1::1

# host2のリンクの設定
ip netns exec host2 ip addr add 2001:db8:0:1::1/64 dev host2-host1
ip netns exec host2 ip link set host2-host1 up
ip netns exec host2 ethtool -K host2-host1 rx off tx off
# ip netns exec router1 sysctl -w net.ipv6.icmp.echo_ignore_all=1
