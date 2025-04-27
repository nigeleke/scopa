### Scopa

## This file is the reference file for all translations for the Scopa application.
## See the [Fluent Syntax Guide](https://projectfluent.org/fluent/guide/index.html)
## for instructions on editing the `.ftl` files.

## Translations used on all pages.

# Application title at the top of all pages;
# Copyright at footer of all pages;
# Version number, e.g. v 1.42.0
scopa-app =
    .title-text = Scopa Scorer
    .copyright-text = Copyright 2025 © Nigel Eke; All rights reserved.
    .version-text = v {$version}

# Points editor; determine number of points to play to.
# Displayed as "<points_editor_prefix> <points edit control> <points_editor_prefix>"
points-editor =
    .prefix = Play to
    .suffix = points
    .aria-label = Enter game target

# Points; status showing points being played to in current game.
points-view =
    .text = Playing to {$n} points

# Team name editor; allow teams to be created during initialisation of game.
team-name-editor =
    .placeholder = Add 2, 3, 4 or 6 teams
    .aria-label = Enter team name

# Start button
start-button =
    .text = Start

# Hints on the icons displayed during scoring.
scopa-icon =
    .hint = Scopa
    .alt-text = Scopa icon

cards-count-icon =
    .hint = Cards count
    .alt-text = Cards count icon

coins-count-icon =
    .hint = Coins count
    .alt-text = Coins count icon

settebello-icon =
    .hint = Settebello
    .alt-text = Settebello icon

premiera-icon =
    .hint = Premiera
    .alt-text = Premiera icon

# Scopa editor; used to score number of Scopas during game play.
score-scopa-editor =
    .aria-label = Scopa score for {$teamname}

# Score group icons; used to select who won cards-count, coins-count, settebello & premiera.
score-group-icon =
    .aria-label = {$group} for {$teamname}

# Round number; status showing current round number.
# Displayed as "<round_text> <round number>"
round-view =
    .text = Round

# Score button.
score-button =
    .text = Score points

# Undo button.
undo-button =
    .text = Undo

# Winner; display the winning team name.
winner-view =
    .text = Winner - {$teamname}

# Start new game button
start-new-game-button =
    .text = Start again

# Start new game settings checkbox
start-new-game-settings =
    .text = Same teams
    .aria-label = Select to use same teams

# Icon
menu-icon =
    .alt-text = Menu
restart-icon =
    .alt-text = Restart

# Supported langauges
lang =
    .en-GB = English
    .it-IT = Italian

# Reset button.
reset-button =
    .text = Reset

# Error instructions.
error =
    .apology = An unexpected error occurred
    .report0 = Please raise an issue on the
    .report1 = issues page
    .report2 = to let us know what happened.

# Not found page
not-found =
    .heading = 404 - Not Found
    .home = Home
