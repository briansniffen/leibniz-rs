import asyncio

limit = 10 ** -6

# starting conditions for Leibniz's approximation of pi
x = 0
d = 1

# a starting guess at how many runs of the computer() inner loop
# will work out to `target` yields per second
ticks=100000

# how many yields actually happened?
tocks=0

# how many yields should computer() do per run of inspector()?
# that is, per second?
target=10


async def inspector():
	global ticks,tocks
	while True:
		await asyncio.sleep(1)
		if tocks<=target:
			ticks = ticks / 2
		elif tocks==target:
			pass
		else:
			ticks = int(ticks * 1.1)
		print(ticks,tocks,d,4*x)
		tocks=0

## TODO: set a target number of digits, and when that's stable, exit cleanly.

@asyncio.coroutine
def computer():
	global d,x,ticks,tocks
	clock=0
	while True:
		x += 1/d
		d += 2
		x -= 1/d
		d += 2
		if clock > ticks:
			tocks += 1
			clock=0
			yield
		else:
			clock+=1

async def cleanup():
	await asyncio.sleep(0.2)

# This is an excessive amount of work on cleanup.  It's a mix of an attempt to be
# careful, cancelling exactly those tasks that need to go---this didn't
# work---and a simple process of making a new event loop and closing off
# the old one.

# I'd love to understand more about where these "task exception was never
# retrieved" errors come from, and how to run the task long enough to process
# the exception.

async def main():
	try:
		c=computer()
		i=inspector()
		await asyncio.gather(c,i)
	finally:
		#c.cancel()
		#i.cancel()
		await cleanup()

asyncio.set_event_loop(asyncio.new_event_loop())
loop = asyncio.get_event_loop()
loop.run_until_complete(main())
loop.close()
