freinn
Sergio I can't replicate the issue in a few days
S
Sergio (IRC)
freinn-M: With Linode, you mean?

freinn
cause I'm at house now and was in the hospital
maybe
and linode stole me 10€
S
Sergio (IRC)
Oh, no, I'm sorry to hear that. :(

freinn
for using nothing
...........
I will switch to digitalocean
S
Sergio (IRC)
I use vultr myself

freinn
a vultr guide so?
will be very nice to have
I'm here just to not be a wordpress guy
but to travel in a rocket u know
S
Sergio (IRC)
Haha
sure
Well, the VPS provider doesn't really matter

freinn
but iptables?
S
Sergio (IRC)
What matters is really the OS/distro you're using.

freinn
ubuntu 16.04
I've tried to open ports with iptables
but the site didn't responded
S
Sergio (IRC)
I mean, port 80 should be open by default.
If you're not familiar with iptables, it's much easier to use ufw.

freinn
I was using rocket alone, no nginx no nothing
S
Sergio (IRC)
Then it would just be `ufw allow http`
`ufw allow https`
S
`ufw enable`

freinn
omg
20:15
magic?
20:15
xD

the problem is that I couldn't launch rocket as root
maybe because of the sudo -E
S
Sergio (IRC)
You shouldn't need `-E`.
So, wait, that worked?

freinn
only on localhost
not on linode
ROCKET_ENV=production ROCKET_PORT=80 sudo -E /home/freinn/.cargo/bin/cargo run
S
Sergio (IRC)
I believe `sudo ROCKET_ENV=production ROCKET_PORT=80 /home/freinn/.cargo/bin/cargo run` should work.
But, you don't need the `ROCKET_PORT`.
S
production port is already 80

freinn
20:19
ok