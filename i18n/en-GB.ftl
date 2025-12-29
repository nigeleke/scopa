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
    .copyright-text = Copyright 2025-2026 © Nigel Eke; All rights reserved.
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

# Icons
menu-icon =
    .alt-text = Menu
restart-icon =
    .alt-text = Restart
help-icon =
    .alt-text = Help
home-icon =
    .alt-text = Home
undo-icon =
    .alt-text = Undo

# Supported langauges
lang =
    .en-GB = English
    .it-IT = Italian

# Reset button.
reset-button =
    .text = Reset

# Cancel button.
cancel-button =
    .text = Cancel

# More button.
more-button =
    .text = More...

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

# Help page
help =
    .heading = Welcome
    .intro =
        Welcome to Scopa Scorer, a companion for your Scopa card game nights!
        You can track scores, manage rounds, and keep the fun flowing with this helpful utility.
    .rules-heading = Rules
    .rules-teams =
        For 2, 3, 4 or 6 players. Four players can play as two teams of two
        or individually. Six players can play as three teams of two, two
        teams of three or individually.
    .rules-deal =
        Deal three cards to each player. Deal four cards in the centre of the table, face up.
    .rules-aim =
        During play the main aims are to a) collect as many cards as possible, b) collect as many
        coin cards as possible, c) collect the seven of coins (Settebello) and d) collect all the
        current cards on the table in a single play (Scopa).
    .rules-play-1 = Lay a card.
    .rules-play-2 =
            If the face value matches any other single card on the table, collect them
            both and put them aside for later scoring.
    .rules-play-3 =
            If the face value doesn't match a single card then you can match more then one
            card. Example, if a 7, 5, 4 and 3 are one the table and you play another 7, you
            must take the 7 on the table and not the 4 & 3. However if you play an 8 instead,
            then you can take the 5 & 3. Note: you don't have to play the 7 but, if you do
            then you have to take that other 7.
    .rules-play-4 =
            If the face value of the card you play is the total of all the other cards, then
            collect them for a Scopa, and record the fact that you also got a Scopa for later
            scoring. It is normal to collect cards in a face down pile, but record each Scopa
            with a single card face up.
    .rules-play-5 =
            If the previous player got a Scopa, the next player simply has to play a card.
            There is not any re-deal of another four cards and the next player has no chance
            to collect.
    .rules-play-6 =
            When all plays have been made, any cards on the table are awarded to the last
            player to have collected cards. This does not count as a Scopa.
    .starting-heading = Set teams & target points
    .starting-intro =
        Before scoring starts, the initial screen allows you to set the target
        for a winning game and add the names the teams or people playing.
    .starting-points =
        Click the target to set the target points for the game.
    .starting-add-team =
        Enter a team name, then click on the '+' to add that team to the list
        of those playing. If you're playing in pairs, only a single team should
        be added, for example "Abbot & Costello".
    .starting-remove-team =
        Click on the '-' to remove teams before the scoring starts.
    .starting-start-game =
        Select [start] to begin scoring the first round of the game.
    .scoring-heading = Scoring rounds
    .scoring-intro =
        The scoring screen allows each round to be scored, until a person or team is declared the winner.
    .scoring-scopa =
        Score the scopas by entering the number of scopas made by each team in their scopa count box.
        This count is only for the current round being scored.
    .scoring-basics =
        To score the "cards count", "coins count", "settebello" & "premiere" select the icon
        under the team.
        "No-one" (the left most icon) can be selected for "cards count", "coins count" and
        "premiere", but a team must be selected for the "settebello".
    .scoring-undo =
        After the first round it is possible to rollback, or undo a score if mistakes are made.
        After the rollback, all rounds need to be manually reapplied.
    .home = Home
