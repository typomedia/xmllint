build:
	cargo build --release
	
compress:
	upx target/release/xmllint