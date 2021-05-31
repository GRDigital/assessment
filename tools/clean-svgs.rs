//# command-macros = { version = "*", features = ["nightly"] }
//# anyhow = "*"

#![feature(proc_macro_hygiene)]
use command_macros::command as cmd;
use std::env::vars as envs;

// npm install -g svgo
fn main() -> anyhow::Result<()> {
	for (icon, svgo_extra) in ICONS {
		println!("{}", icon);
		cmd!(cargo play --release -q tools/svg_cleaner.rs -- -i (icon)).envs(envs()).status()?;
		cmd!(cmd /c svgo --pretty --multipass --config=tools/svgo.yml [svgo_extra.iter()] (icon)).envs(envs()).status()?;
		cmd!(cargo play --release -q tools/svg_cleaner.rs -- -i (icon)).envs(envs()).status()?;
	}

	Ok(())
}

// icon_name, svgo_extra, svgcleaner_extra
// svgo --disable=removeXMLNS for icons to work as base64 css
// svgo --disable=mergePaths to be able to colour different parts independently
static ICONS: &'static [(&'static str, &'static [&'static str])] = &[
	("public/img/icons/arrows/arrow-bold-bottom.svg", &[]),
	("public/img/icons/arrows/arrow-bold-forvard-all.svg", &[]),
	("public/img/icons/arrows/arrow-bold-forvard.svg", &[]),
	("public/img/icons/arrows/arrow-bold-left.svg", &[]),
	("public/img/icons/arrows/arrow-bold-redo.svg", &[]),
	("public/img/icons/arrows/arrow-bold-reply-all.svg", &[]),
	("public/img/icons/arrows/arrow-bold-reply.svg", &[]),
	("public/img/icons/arrows/arrow-bold-right.svg", &[]),
	("public/img/icons/arrows/arrow-bold-top.svg", &[]),
	("public/img/icons/arrows/arrow-bold-undo.svg", &[]),
	("public/img/icons/arrows/arrow-bottom-left.svg", &[]),
	("public/img/icons/arrows/arrow-bottom-right.svg", &[]),
	("public/img/icons/arrows/arrow-bottom.svg", &[]),
	("public/img/icons/arrows/arrow-forward-all.svg", &[]),
	("public/img/icons/arrows/arrow-forward.svg", &[]),
	("public/img/icons/arrows/arrow-left-curved.svg", &[]),
	("public/img/icons/arrows/arrow-left.svg", &[]),
	("public/img/icons/arrows/arrow-redo.svg", &[]),
	("public/img/icons/arrows/arrow-reply-all.svg", &[]),
	("public/img/icons/arrows/arrow-reply.svg", &[]),
	("public/img/icons/arrows/arrow-right-curved.svg", &[]),
	("public/img/icons/arrows/arrow-right.svg", &[]),
	("public/img/icons/arrows/arrow-top-left.svg", &[]),
	("public/img/icons/arrows/arrow-top-right.svg", &[]),
	("public/img/icons/arrows/arrow-top.svg", &[]),
	("public/img/icons/arrows/arrow-undo.svg", &[]),
	("public/img/icons/arrows/arrows-diagonals-bltr.svg", &[]),
	("public/img/icons/arrows/arrows-diagonals-tlbr.svg", &[]),
	("public/img/icons/arrows/arrows-diagonals.svg", &[]),
	("public/img/icons/arrows/arrows-hv.svg", &[]),
	("public/img/icons/arrows/chevron-bottom.svg", &[]),
	("public/img/icons/arrows/chevron-left.svg", &[]),
	("public/img/icons/arrows/chevron-right.svg", &[]),
	("public/img/icons/arrows/chevron-top.svg", &[]),
	("public/img/icons/arrows/chevrons-bottom.svg", &[]),
	("public/img/icons/arrows/chevrons-left.svg", &[]),
	("public/img/icons/arrows/chevrons-right.svg", &[]),
	("public/img/icons/arrows/chevrons-top.svg", &[]),
	("public/img/icons/arrows/circle-arrow-bottom-left.svg", &[]),
	("public/img/icons/arrows/circle-arrow-bottom-right.svg", &[]),
	("public/img/icons/arrows/circle-arrow-bottom.svg", &[]),
	("public/img/icons/arrows/circle-arrow-left-curved.svg", &[]),
	("public/img/icons/arrows/circle-arrow-left.svg", &[]),
	("public/img/icons/arrows/circle-arrow-right-curved.svg", &[]),
	("public/img/icons/arrows/circle-arrow-right.svg", &[]),
	("public/img/icons/arrows/circle-arrow-top-left.svg", &[]),
	("public/img/icons/arrows/circle-arrow-top-right.svg", &[]),
	("public/img/icons/arrows/circle-arrow-top.svg", &[]),
	("public/img/icons/arrows/circle-chevron-bottom.svg", &[]),
	("public/img/icons/arrows/circle-chevron-left.svg", &[]),
	("public/img/icons/arrows/circle-chevron-right.svg", &[]),
	("public/img/icons/arrows/circle-chevron-top.svg", &[]),
	("public/img/icons/arrows/circle-chevrons-bottom.svg", &[]),
	("public/img/icons/arrows/circle-chevrons-left.svg", &[]),
	("public/img/icons/arrows/circle-chevrons-right.svg", &[]),
	("public/img/icons/arrows/circle-chevrons-top.svg", &[]),
	("public/img/icons/arrows/refresh-ccw-alert.svg", &[]),
	("public/img/icons/arrows/refresh-ccw.svg", &[]),
	("public/img/icons/arrows/refresh-cw-alert.svg", &[]),
	("public/img/icons/arrows/refresh-cw.svg", &[]),
	("public/img/icons/arrows/rotate-ccw.svg", &[]),
	("public/img/icons/arrows/rotate-cw.svg", &[]),
	("public/img/icons/call/call-add.svg", &[]),
	("public/img/icons/call/call-calling.svg", &[]),
	("public/img/icons/call/call-decline.svg", &[]),
	("public/img/icons/call/call-end.svg", &[]),
	("public/img/icons/call/call-forwarded.svg", &[]),
	("public/img/icons/call/call-hash.svg", &[]),
	("public/img/icons/call/call-hold.svg", &[]),
	("public/img/icons/call/call-incoming.svg", &[]),
	("public/img/icons/call/call-missed.svg", &[]),
	("public/img/icons/call/call-muted.svg", &[]),
	("public/img/icons/call/call-no.svg", &[]),
	("public/img/icons/call/call-numbers.svg", &[]),
	("public/img/icons/call/call-outcoming.svg", &[]),
	("public/img/icons/call/call-phone.svg", &[]),
	("public/img/icons/call/call-recieved.svg", &[]),
	("public/img/icons/call/call-voicemail.svg", &[]),
	("public/img/icons/chatting/comment-checked.svg", &[]),
	("public/img/icons/chatting/comment-delete.svg", &[]),
	("public/img/icons/chatting/comment-minus.svg", &[]),
	("public/img/icons/chatting/comment-plus.svg", &[]),
	("public/img/icons/chatting/comment-text.svg", &[]),
	("public/img/icons/chatting/comment.svg", &[]),
	("public/img/icons/chatting/question.svg", &[]),
	("public/img/icons/connection/airplay.svg", &[]),
	("public/img/icons/connection/bluetooth-no.svg", &[]),
	("public/img/icons/connection/bluetooth.svg", &[]),
	("public/img/icons/connection/broadcast.svg", &[]),
	("public/img/icons/connection/broadcasting.svg", &[]),
	("public/img/icons/connection/cast.svg", &[]),
	("public/img/icons/connection/wi-fi-no.svg", &[]),
	("public/img/icons/connection/wi-fi.svg", &[]),
	("public/img/icons/file/clipboard-checked.svg", &[]),
	("public/img/icons/file/clipboard-delete.svg", &[]),
	("public/img/icons/file/clipboard-minus.svg", &[]),
	("public/img/icons/file/clipboard-plus.svg", &[]),
	("public/img/icons/file/clipboard-text.svg", &[]),
	("public/img/icons/file/clipboard.svg", &[]),
	("public/img/icons/file/file-checked.svg", &[]),
	("public/img/icons/file/file-code.svg", &[]),
	("public/img/icons/file/file-create.svg", &[]),
	("public/img/icons/file/file-delete.svg", &[]),
	("public/img/icons/file/file-download.svg", &[]),
	("public/img/icons/file/file-draft.svg", &[]),
	("public/img/icons/file/file-minus.svg", &[]),
	("public/img/icons/file/file-plus.svg", &[]),
	("public/img/icons/file/file-scan.svg", &[]),
	("public/img/icons/file/file-shredder.svg", &[]),
	("public/img/icons/file/file-text.svg", &[]),
	("public/img/icons/file/file-upload.svg", &[]),
	("public/img/icons/file/file.svg", &[]),
	("public/img/icons/file/folder-arrow.svg", &[]),
	("public/img/icons/file/folder-checked.svg", &[]),
	("public/img/icons/file/folder-cloud.svg", &[]),
	("public/img/icons/file/folder-create.svg", &[]),
	("public/img/icons/file/folder-delete.svg", &[]),
	("public/img/icons/file/folder-minus.svg", &[]),
	("public/img/icons/file/folder-music.svg", &[]),
	("public/img/icons/file/folder-opened.svg", &[]),
	("public/img/icons/file/folder-photo.svg", &[]),
	("public/img/icons/file/folder-plus.svg", &[]),
	("public/img/icons/file/folder-zip.svg", &[]),
	("public/img/icons/file/folder.svg", &[]),
	("public/img/icons/file/note-text.svg", &[]),
	("public/img/icons/file/note.svg", &[]),
	("public/img/icons/finance/bitcoin.svg", &[]),
	("public/img/icons/finance/creditcard-add.svg", &[]),
	("public/img/icons/finance/creditcard-face.svg", &[]),
	("public/img/icons/finance/creditcard-income.svg", &[]),
	("public/img/icons/finance/creditcard-no.svg", &[]),
	("public/img/icons/finance/creditcard-outcome.svg", &[]),
	("public/img/icons/finance/creditcard-scan.svg", &[]),
	("public/img/icons/finance/creditcard.svg", &[]),
	("public/img/icons/finance/ethereum.svg", &[]),
	("public/img/icons/finance/paypass.svg", &[]),
	("public/img/icons/finance/strongbox.svg", &[]),
	("public/img/icons/finance/wallet.svg", &[]),
	("public/img/icons/food/avocado.svg", &[]),
	("public/img/icons/food/bread.svg", &[]),
	("public/img/icons/food/chicken.svg", &[]),
	("public/img/icons/food/coffee.svg", &[]),
	("public/img/icons/food/egg.svg", &[]),
	("public/img/icons/food/icecream.svg", &[]),
	("public/img/icons/gadgets/battery-100.svg", &[]),
	("public/img/icons/gadgets/battery-20.svg", &[]),
	("public/img/icons/gadgets/battery-40.svg", &[]),
	("public/img/icons/gadgets/battery-60.svg", &[]),
	("public/img/icons/gadgets/battery-80.svg", &[]),
	("public/img/icons/gadgets/battery-charching.svg", &[]),
	("public/img/icons/gadgets/battery-no.svg", &[]),
	("public/img/icons/gadgets/battery.svg", &[]),
	("public/img/icons/gadgets/devices.svg", &[]),
	("public/img/icons/gadgets/flash-card.svg", &[]),
	("public/img/icons/gadgets/floppy.svg", &[]),
	("public/img/icons/gadgets/iPad.svg", &[]),
	("public/img/icons/gadgets/iPhone.svg", &[]),
	("public/img/icons/gadgets/laptop.svg", &[]),
	("public/img/icons/gadgets/memory-card.svg", &[]),
	("public/img/icons/gadgets/mobile.svg", &[]),
	("public/img/icons/gadgets/monitor.svg", &[]),
	("public/img/icons/gadgets/server.svg", &[]),
	("public/img/icons/grid/apps.svg", &[]),
	("public/img/icons/grid/block-align-bottom.svg", &[]),
	("public/img/icons/grid/block-align-horizontally.svg", &[]),
	("public/img/icons/grid/block-align-left.svg", &[]),
	("public/img/icons/grid/block-align-right.svg", &[]),
	("public/img/icons/grid/block-align-top.svg", &[]),
	("public/img/icons/grid/block-align-vertically.svg", &[]),
	("public/img/icons/grid/block-distribute-horizontally.svg", &[]),
	("public/img/icons/grid/block-distribute-vertically.svg", &[]),
	("public/img/icons/grid/brake-page.svg", &[]),
	("public/img/icons/grid/edit-shape.svg", &[]),
	("public/img/icons/grid/elements.svg", &[]),
	("public/img/icons/grid/frame.svg", &[]),
	("public/img/icons/grid/grid-6.svg", &[]),
	("public/img/icons/grid/grid-col-2.svg", &[]),
	("public/img/icons/grid/grid-col-3.svg", &[]),
	("public/img/icons/grid/grid-dynamic.svg", &[]),
	("public/img/icons/grid/grid-frame.svg", &[]),
	("public/img/icons/grid/grid-row-2.svg", &[]),
	("public/img/icons/grid/grid-row-3.svg", &[]),
	("public/img/icons/grid/grid-row-alt.svg", &[]),
	("public/img/icons/grid/grid-slides.svg", &[]),
	("public/img/icons/grid/grid-small.svg", &[]),
	("public/img/icons/grid/grid.svg", &[]),
	("public/img/icons/grid/iframe.svg", &[]),
	("public/img/icons/grid/kanban.svg", &[]),
	("public/img/icons/grid/layout.svg", &[]),
	("public/img/icons/grid/padding.svg", &[]),
	("public/img/icons/grid/rotate-left.svg", &[]),
	("public/img/icons/grid/rotate-right.svg", &[]),
	("public/img/icons/grid/ruller.svg", &[]),
	("public/img/icons/grid/segment.svg", &[]),
	("public/img/icons/grid/select-area.svg", &[]),
	("public/img/icons/grid/select.svg", &[]),
	("public/img/icons/grid/sidebar-left.svg", &[]),
	("public/img/icons/grid/sidebar-right.svg", &[]),
	("public/img/icons/grid/stack-1.svg", &[]),
	("public/img/icons/grid/stack.svg", &[]),
	("public/img/icons/grid/text.svg", &[]),
	("public/img/icons/maps/360.svg", &[]),
	("public/img/icons/maps/compas.svg", &[]),
	("public/img/icons/maps/direction-45.svg", &[]),
	("public/img/icons/maps/direction.svg", &[]),
	("public/img/icons/maps/globe.svg", &[]),
	("public/img/icons/maps/location-no.svg", &[]),
	("public/img/icons/maps/location.svg", &[]),
	("public/img/icons/maps/map-pin-location.svg", &[]),
	("public/img/icons/maps/map.svg", &[]),
	("public/img/icons/maps/panorama.svg", &[]),
	("public/img/icons/maps/pin-add.svg", &[]),
	("public/img/icons/maps/pin-no.svg", &[]),
	("public/img/icons/maps/pin-question.svg", &[]),
	("public/img/icons/maps/pin-round.svg", &[]),
	("public/img/icons/maps/pin-start.svg", &[]),
	("public/img/icons/maps/pin.svg", &[]),
	("public/img/icons/maps/radar.svg", &[]),
	("public/img/icons/maps/route.svg", &[]),
	("public/img/icons/music/CD.svg", &[]),
	("public/img/icons/music/add-to-library.svg", &[]),
	("public/img/icons/music/artist.svg", &[]),
	("public/img/icons/music/eject.svg", &[]),
	("public/img/icons/music/equalizer.svg", &[]),
	("public/img/icons/music/listen-later.svg", &[]),
	("public/img/icons/music/music-library.svg", &[]),
	("public/img/icons/music/music-note.svg", &[]),
	("public/img/icons/music/player-fast-back.svg", &[]),
	("public/img/icons/music/player-fast-forward.svg", &[]),
	("public/img/icons/music/player-list-add.svg", &[]),
	("public/img/icons/music/player-list-play.svg", &[]),
	("public/img/icons/music/player-list.svg", &[]),
	("public/img/icons/music/player-pause-circle.svg", &[]),
	("public/img/icons/music/player-pause.svg", &[]),
	("public/img/icons/music/player-play-circle.svg", &[]),
	("public/img/icons/music/player-play-pause.svg", &[]),
	("public/img/icons/music/player-play.svg", &[]),
	("public/img/icons/music/player-skip-back.svg", &[]),
	("public/img/icons/music/player-skip-forward.svg", &[]),
	("public/img/icons/music/player-stop-circle.svg", &[]),
	("public/img/icons/music/player-stop.svg", &[]),
	("public/img/icons/music/queue.svg", &[]),
	("public/img/icons/music/radio.svg", &[]),
	("public/img/icons/music/record.svg", &[]),
	("public/img/icons/music/repeat.svg", &[]),
	("public/img/icons/music/shuffle.svg", &[]),
	("public/img/icons/music/sound-wave.svg", &[]),
	("public/img/icons/music/tuner.svg", &[]),
	("public/img/icons/music/volume-high.svg", &[]),
	("public/img/icons/music/volume-low.svg", &[]),
	("public/img/icons/music/volume-no.svg", &[]),
	("public/img/icons/music/volume-off.svg", &[]),
	("public/img/icons/music/volume.svg", &[]),
	("public/img/icons/notifications/alert-circle.svg", &[]),
	("public/img/icons/notifications/alert-octagon.svg", &[]),
	("public/img/icons/notifications/alert-triangle.svg", &[]),
	("public/img/icons/notifications/app-notification.svg", &[]),
	("public/img/icons/notifications/bell-add.svg", &[]),
	("public/img/icons/notifications/bell-alert.svg", &[]),
	("public/img/icons/notifications/bell-checked.svg", &[]),
	("public/img/icons/notifications/bell-minus.svg", &[]),
	("public/img/icons/notifications/bell-no.svg", &[]),
	("public/img/icons/notifications/bell.svg", &[]),
	("public/img/icons/notifications/info.svg", &[]),
	("public/img/icons/notifications/minus-octagon.svg", &[]),
	("public/img/icons/notifications/question-circle.svg", &[]),
	("public/img/icons/notifications/x-octagon.svg", &[]),
	("public/img/icons/reactions/laughing.svg", &[]),
	("public/img/icons/reactions/neutral.svg", &[]),
	("public/img/icons/reactions/sad.svg", &[]),
	("public/img/icons/reactions/shocked.svg", &[]),
	("public/img/icons/reactions/smiled.svg", &[]),
	("public/img/icons/security/key-no.svg", &[]),
	("public/img/icons/security/key.svg", &[]),
	("public/img/icons/security/lock-circle.svg", &[]),
	("public/img/icons/security/lock-no.svg", &[]),
	("public/img/icons/security/lock.svg", &[]),
	("public/img/icons/security/shield-lock.svg", &[]),
	("public/img/icons/security/shield-no.svg", &[]),
	("public/img/icons/security/shield-ok.svg", &[]),
	("public/img/icons/security/shield.svg", &[]),
	("public/img/icons/security/unlock.svg", &[]),
	("public/img/icons/security/verified.svg", &[]),
	("public/img/icons/security/verified2.svg", &[]),
	("public/img/icons/shopping/bag.svg", &[]),
	("public/img/icons/shopping/bascket.svg", &[]),
	("public/img/icons/shopping/cart.svg", &[]),
	("public/img/icons/shopping/coupon.svg", &[]),
	("public/img/icons/shopping/cut-coupon.svg", &[]),
	("public/img/icons/shopping/delivery.svg", &[]),
	("public/img/icons/shopping/discount.svg", &[]),
	("public/img/icons/shopping/filter.svg", &[]),
	("public/img/icons/shopping/gift.svg", &[]),
	("public/img/icons/shopping/pos.svg", &[]),
	("public/img/icons/shopping/tag.svg", &[]),
	("public/img/icons/shopping/tote.svg", &[]),
	("public/img/icons/software/cloud-checked.svg", &[]),
	("public/img/icons/software/cloud-connect.svg", &[]),
	("public/img/icons/software/cloud-download.svg", &[]),
	("public/img/icons/software/cloud-no.svg", &[]),
	("public/img/icons/software/cloud-upload.svg", &[]),
	("public/img/icons/software/cloud.svg", &[]),
	("public/img/icons/software/code.svg", &[]),
	("public/img/icons/software/database.svg", &[]),
	("public/img/icons/software/terminal.svg", &[]),
	("public/img/icons/symbols/at-sign.svg", &[]),
	("public/img/icons/symbols/behance.svg", &[]),
	("public/img/icons/symbols/cc.svg", &[]),
	("public/img/icons/symbols/cc0.svg", &[]),
	("public/img/icons/symbols/command.svg", &[]),
	("public/img/icons/symbols/dribbble.svg", &[]),
	("public/img/icons/symbols/facebook.svg", &[]),
	("public/img/icons/symbols/github.svg", &[]),
	("public/img/icons/symbols/gitlab.svg", &[]),
	("public/img/icons/symbols/google.svg", &[]),
	("public/img/icons/symbols/hash.svg", &[]),
	("public/img/icons/symbols/instagram.svg", &[]),
	("public/img/icons/symbols/linkedin.svg", &[]),
	("public/img/icons/symbols/peace.svg", &[]),
	("public/img/icons/symbols/twitter.svg", &[]),
	("public/img/icons/symbols/youtube.svg", &[]),
	("public/img/icons/time/alarm-checked.svg", &[]),
	("public/img/icons/time/alarm-minus.svg", &[]),
	("public/img/icons/time/alarm-no.svg", &[]),
	("public/img/icons/time/alarm-plus.svg", &[]),
	("public/img/icons/time/alarm-snooze.svg", &[]),
	("public/img/icons/time/alarm.svg", &[]),
	("public/img/icons/time/calendar-checked.svg", &[]),
	("public/img/icons/time/calendar-create.svg", &[]),
	("public/img/icons/time/calendar-dates.svg", &[]),
	("public/img/icons/time/calendar-delete.svg", &[]),
	("public/img/icons/time/calendar-minus.svg", &[]),
	("public/img/icons/time/calendar-plus.svg", &[]),
	("public/img/icons/time/calendar.svg", &[]),
	("public/img/icons/time/stopwatch.svg", &[]),
	("public/img/icons/time/time-history.svg", &[]),
	("public/img/icons/time/time.svg", &[]),
	("public/img/icons/time/timer.svg", &[]),
	("public/img/icons/time/watch.svg", &[]),
	("public/img/icons/various/advertisement.svg", &[]),
	("public/img/icons/various/atom.svg", &[]),
	("public/img/icons/various/bone.svg", &[]),
	("public/img/icons/various/brightness-high.svg", &[]),
	("public/img/icons/various/brightness-low.svg", &[]),
	("public/img/icons/various/contrast.svg", &[]),
	("public/img/icons/various/cross.svg", &[]),
	("public/img/icons/various/crosshairs.svg", &[]),
	("public/img/icons/various/cup.svg", &[]),
	("public/img/icons/various/form.svg", &[]),
	("public/img/icons/various/infinity.svg", &[]),
	("public/img/icons/various/items.svg", &[]),
	("public/img/icons/various/lightbulb.svg", &[]),
	("public/img/icons/various/moon.svg", &[]),
	("public/img/icons/various/nut.svg", &[]),
	("public/img/icons/various/planet.svg", &[]),
	("public/img/icons/various/pocket.svg", &[]),
	("public/img/icons/various/rocket.svg", &[]),
	("public/img/icons/various/sun.svg", &[]),
	("public/img/icons/various/toy-horse.svg", &[]),
	("public/img/icons/activity.svg", &[]),
	("public/img/icons/airplay.svg", &[]),
	("public/img/icons/alert-circle.svg", &[]),
	("public/img/icons/alert-octagon.svg", &[]),
	("public/img/icons/alert-triangle.svg", &[]),
	("public/img/icons/align-center.svg", &[]),
	("public/img/icons/align-justify.svg", &[]),
	("public/img/icons/align-left.svg", &[]),
	("public/img/icons/align-right.svg", &[]),
	("public/img/icons/anchor.svg", &[]),
	("public/img/icons/aperture.svg", &[]),
	("public/img/icons/arrow-down-left.svg", &[]),
	("public/img/icons/arrow-down-right.svg", &[]),
	("public/img/icons/arrow-down.svg", &[]),
	("public/img/icons/arrow-left.svg", &[]),
	("public/img/icons/arrow-right.svg", &[]),
	("public/img/icons/arrow-up-left.svg", &[]),
	("public/img/icons/arrow-up-right.svg", &[]),
	("public/img/icons/arrow-up.svg", &[]),
	("public/img/icons/arrow-button.svg", &[]),
	("public/img/icons/at-sign.svg", &[]),
	("public/img/icons/award.svg", &[]),
	("public/img/icons/bar-chart-2.svg", &[]),
	("public/img/icons/bar-chart.svg", &[]),
	("public/img/icons/battery-charging.svg", &[]),
	("public/img/icons/battery.svg", &[]),
	("public/img/icons/bell-off.svg", &[]),
	("public/img/icons/bell.svg", &[]),
	("public/img/icons/blockquote.svg", &[]),
	("public/img/icons/bluetooth.svg", &[]),
	("public/img/icons/bold.svg", &[]),
	("public/img/icons/book.svg", &[]),
	("public/img/icons/bookmark.svg", &[]),
	("public/img/icons/box.svg", &[]),
	("public/img/icons/briefcase.svg", &[]),
	("public/img/icons/calendar.svg", &[]),
	("public/img/icons/camera-off.svg", &[]),
	("public/img/icons/camera.svg", &[]),
	("public/img/icons/cast.svg", &[]),
	("public/img/icons/check-circle.svg", &[]),
	("public/img/icons/check-square.svg", &[]),
	("public/img/icons/check.svg", &[]),
	("public/img/icons/chevron-down.svg", &[]),
	("public/img/icons/chevron-left.svg", &[]),
	("public/img/icons/chevron-right.svg", &[]),
	("public/img/icons/chevron-up.svg", &[]),
	("public/img/icons/chevrons-down.svg", &[]),
	("public/img/icons/chevrons-left.svg", &[]),
	("public/img/icons/chevrons-right.svg", &[]),
	("public/img/icons/chevrons-up.svg", &[]),
	("public/img/icons/chrome.svg", &[]),
	("public/img/icons/circle.svg", &[]),
	("public/img/icons/clipboard.svg", &[]),
	("public/img/icons/clock.svg", &[]),
	("public/img/icons/cloud-drizzle.svg", &[]),
	("public/img/icons/cloud-lightning.svg", &[]),
	("public/img/icons/cloud-off.svg", &[]),
	("public/img/icons/cloud-rain.svg", &[]),
	("public/img/icons/cloud-snow.svg", &[]),
	("public/img/icons/cloud.svg", &[]),
	("public/img/icons/codepen.svg", &[]),
	("public/img/icons/command.svg", &[]),
	("public/img/icons/compass.svg", &[]),
	("public/img/icons/copy.svg", &[]),
	("public/img/icons/corner-down-left.svg", &[]),
	("public/img/icons/corner-down-right.svg", &[]),
	("public/img/icons/corner-left-down.svg", &[]),
	("public/img/icons/corner-left-up.svg", &[]),
	("public/img/icons/corner-right-down.svg", &[]),
	("public/img/icons/corner-right-up.svg", &[]),
	("public/img/icons/corner-up-left.svg", &[]),
	("public/img/icons/corner-up-right.svg", &[]),
	("public/img/icons/cpu.svg", &[]),
	("public/img/icons/credit-card.svg", &[]),
	("public/img/icons/crop.svg", &[]),
	("public/img/icons/crosshair.svg", &[]),
	("public/img/icons/delete.svg", &[]),
	("public/img/icons/disc.svg", &[]),
	("public/img/icons/download-cloud.svg", &[]),
	("public/img/icons/download.svg", &[]),
	("public/img/icons/droplet.svg", &[]),
	("public/img/icons/edit-2.svg", &[]),
	("public/img/icons/edit-3.svg", &[]),
	("public/img/icons/edit.svg", &[]),
	("public/img/icons/external-link.svg", &[]),
	("public/img/icons/eye-off.svg", &[]),
	("public/img/icons/eye.svg", &[]),
	("public/img/icons/facebook.svg", &[]),
	("public/img/icons/fast-forward.svg", &[]),
	("public/img/icons/feather.svg", &[]),
	("public/img/icons/file-minus.svg", &[]),
	("public/img/icons/file-plus.svg", &[]),
	("public/img/icons/file-text.svg", &[]),
	("public/img/icons/file.svg", &[]),
	("public/img/icons/film.svg", &[]),
	("public/img/icons/filter.svg", &[]),
	("public/img/icons/flag.svg", &[]),
	("public/img/icons/folder.svg", &[]),
	("public/img/icons/github.svg", &[]),
	("public/img/icons/gitlab.svg", &[]),
	("public/img/icons/globe.svg", &[]),
	("public/img/icons/grid.svg", &[]),
	("public/img/icons/hash.svg", &[]),
	("public/img/icons/headphones.svg", &[]),
	("public/img/icons/heart.svg", &[]),
	("public/img/icons/help-circle.svg", &[]),
	("public/img/icons/home.svg", &[]),
	("public/img/icons/image.svg", &[]),
	("public/img/icons/inbox.svg", &[]),
	("public/img/icons/info.svg", &[]),
	("public/img/icons/instagram.svg", &[]),
	("public/img/icons/italic.svg", &[]),
	("public/img/icons/layers.svg", &[]),
	("public/img/icons/layout.svg", &[]),
	("public/img/icons/life-buoy.svg", &[]),
	("public/img/icons/link-2.svg", &[]),
	("public/img/icons/link.svg", &[]),
	("public/img/icons/list.svg", &[]),
	("public/img/icons/loader.svg", &[]),
	("public/img/icons/lock.svg", &[]),
	("public/img/icons/log-in.svg", &[]),
	("public/img/icons/log-out.svg", &[]),
	("public/img/icons/mail.svg", &[]),
	("public/img/icons/map-pin.svg", &[]),
	("public/img/icons/map.svg", &[]),
	("public/img/icons/maximize-2.svg", &[]),
	("public/img/icons/maximize.svg", &[]),
	("public/img/icons/menu.svg", &[]),
	("public/img/icons/message-circle.svg", &[]),
	("public/img/icons/message-square.svg", &[]),
	("public/img/icons/mic-off.svg", &[]),
	("public/img/icons/mic.svg", &[]),
	("public/img/icons/minimize-2.svg", &[]),
	("public/img/icons/minimize.svg", &[]),
	("public/img/icons/minus-circle.svg", &[]),
	("public/img/icons/minus-square.svg", &[]),
	("public/img/icons/minus.svg", &[]),
	("public/img/icons/monitor.svg", &[]),
	("public/img/icons/moon.svg", &[]),
	("public/img/icons/more-horizontal.svg", &[]),
	("public/img/icons/more-vertical.svg", &[]),
	("public/img/icons/move.svg", &[]),
	("public/img/icons/music.svg", &[]),
	("public/img/icons/navigation-2.svg", &[]),
	("public/img/icons/navigation.svg", &[]),
	("public/img/icons/octagon.svg", &[]),
	("public/img/icons/package.svg", &[]),
	("public/img/icons/paperclip.svg", &[]),
	("public/img/icons/pause-circle.svg", &[]),
	("public/img/icons/pause.svg", &[]),
	("public/img/icons/percent.svg", &[]),
	("public/img/icons/phone-call.svg", &[]),
	("public/img/icons/phone-forwarded.svg", &[]),
	("public/img/icons/phone-incoming.svg", &[]),
	("public/img/icons/phone-missed.svg", &[]),
	("public/img/icons/phone-off.svg", &[]),
	("public/img/icons/phone-outgoing.svg", &[]),
	("public/img/icons/phone.svg", &[]),
	("public/img/icons/pie-chart.svg", &[]),
	("public/img/icons/play-circle.svg", &[]),
	("public/img/icons/play.svg", &[]),
	("public/img/icons/plus-circle.svg", &[]),
	("public/img/icons/plus-square.svg", &[]),
	("public/img/icons/plus.svg", &[]),
	("public/img/icons/pocket.svg", &[]),
	("public/img/icons/power.svg", &[]),
	("public/img/icons/printer.svg", &[]),
	("public/img/icons/radio.svg", &[]),
	("public/img/icons/refresh-ccw.svg", &[]),
	("public/img/icons/refresh-cw.svg", &[]),
	("public/img/icons/repeat.svg", &[]),
	("public/img/icons/resource-bullet.svg", &[]),
	("public/img/icons/rewind.svg", &[]),
	("public/img/icons/rotate-ccw.svg", &[]),
	("public/img/icons/rotate-cw.svg", &[]),
	("public/img/icons/save.svg", &[]),
	("public/img/icons/scissors.svg", &[]),
	("public/img/icons/search.svg", &[]),
	("public/img/icons/server.svg", &[]),
	("public/img/icons/settings.svg", &[]),
	("public/img/icons/share-2.svg", &[]),
	("public/img/icons/share.svg", &[]),
	("public/img/icons/shield.svg", &[]),
	("public/img/icons/shopping-cart.svg", &[]),
	("public/img/icons/shuffle.svg", &[]),
	("public/img/icons/sidebar.svg", &[]),
	("public/img/icons/skip-back.svg", &[]),
	("public/img/icons/skip-forward.svg", &[]),
	("public/img/icons/slack.svg", &[]),
	("public/img/icons/slash.svg", &[]),
	("public/img/icons/sliders.svg", &[]),
	("public/img/icons/smartphone.svg", &[]),
	("public/img/icons/speaker.svg", &[]),
	("public/img/icons/square.svg", &[]),
	("public/img/icons/star.svg", &[]),
	("public/img/icons/stop-circle.svg", &[]),
	("public/img/icons/sun.svg", &[]),
	("public/img/icons/sunrise.svg", &[]),
	("public/img/icons/sunset.svg", &[]),
	("public/img/icons/tablet.svg", &[]),
	("public/img/icons/tag.svg", &[]),
	("public/img/icons/target.svg", &[]),
	("public/img/icons/thermometer.svg", &[]),
	("public/img/icons/thumbs-down.svg", &[]),
	("public/img/icons/thumbs-up.svg", &[]),
	("public/img/icons/toggle-left.svg", &[]),
	("public/img/icons/toggle-right.svg", &[]),
	("public/img/icons/trash-2.svg", &[]),
	("public/img/icons/trash.svg", &[]),
	("public/img/icons/trending-down.svg", &[]),
	("public/img/icons/trending-up.svg", &[]),
	("public/img/icons/triangle.svg", &[]),
	("public/img/icons/tv.svg", &[]),
	("public/img/icons/twitter.svg", &[]),
	("public/img/icons/type.svg", &[]),
	("public/img/icons/umbrella.svg", &[]),
	("public/img/icons/underline.svg", &[]),
	("public/img/icons/unlock.svg", &[]),
	("public/img/icons/upload-cloud.svg", &[]),
	("public/img/icons/upload.svg", &[]),
	("public/img/icons/user-check.svg", &[]),
	("public/img/icons/user-minus.svg", &[]),
	("public/img/icons/user-plus.svg", &[]),
	("public/img/icons/user-x.svg", &[]),
	("public/img/icons/user.svg", &[]),
	("public/img/icons/users.svg", &[]),
	("public/img/icons/video-off.svg", &[]),
	("public/img/icons/video.svg", &[]),
	("public/img/icons/voicemail.svg", &[]),
	("public/img/icons/volume-1.svg", &[]),
	("public/img/icons/volume-2.svg", &[]),
	("public/img/icons/volume-x.svg", &[]),
	("public/img/icons/volume.svg", &[]),
	("public/img/icons/watch.svg", &[]),
	("public/img/icons/wifi-off.svg", &[]),
	("public/img/icons/wifi.svg", &[]),
	("public/img/icons/wind.svg", &[]),
	("public/img/icons/x-circle.svg", &[]),
	("public/img/icons/x-square.svg", &[]),
	("public/img/icons/x.svg", &[]),
	("public/img/icons/zap.svg", &[]),
	("public/img/icons/zoom-in.svg", &[]),
	("public/img/icons/zoom-out.svg", &[]),
];