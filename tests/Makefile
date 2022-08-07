all: test_std test_no_std

test_no_std:
	cd no_std && renode-test no_std.robot -r outputs

test_std:
	robot --outputdir outputs std.robot
