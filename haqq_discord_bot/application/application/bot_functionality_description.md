# Discord user
## !raffle_chance
Gives a chance to win in a raffle.
It is needed to send this message to Bot direct messages.

## !my_stake
Gives total quantity of staked ISLM.
It is needed to send this message to Bot direct messages.

# Discord administrator
## /initiate_raffle  `prize_amount` `winners_number` `duration`
Сreates a raffle with the specified parameters:
1. `prize_amount` - ISLM quantity which will be paid to the raffle winner as a prize. <br> The integer number from 1 to 9223372036854775807. <br> Parameter is required.
2. `winners_number` - maximum number of people who will be chosen as raffle winners. <br> The integer number from 1 to 9223372036854775807. <br> Parameter is required.
3. `duration` - number of `hours` from the current moment during which raffle participants will be recruited. <br> The integer number from 1 to 9223372036854775807. <br> Parameter is required.

The command can be executed from any current Discord-server channel. It is safe to execute this command. It will be possible to execute this command only if all parameters are valid and there is no uncompleted raffle.

## /cancel_raffle
Cancels the current uncompleted raffle. That is, no winners are selected and the raffle completes.

The command can be executed from any current Discord-server channel. It is safe to execute this command. It will be possible to execute this command only if there is uncompleted raffle.

## /complete_raffle
Selects winners and completes current uncomplited raffle.

The command can be executed from any current Discord-server channel. It is safe to execute this command. It will be possible to execute this command only if there is uncompleted raffle.

## /update_raffle  `prize_amount` `winners_number` `duration`
Updates the current uncompleted raffle with the specified parameters:
1. `prize_amount` - ISLM quantity which will be paid to the raffle winner as a prize. <br> The integer number from 1 to 9223372036854775807. <br> Parameter is not required.
2. `winners_number` - maximum number of people who will be chosen as raffle winners. <br> The integer number from 1 to 9223372036854775807. <br> Parameter is not required.
3. `duration` - number of `hours` from the current moment during which raffle participants will be recruited. <br> The integer number from 1 to 9223372036854775807. <br> Parameter is not required.

The command can be executed from any current Discord-server channel. It is safe to execute this command. It will be possible to execute this command only if all parameters are valid, at least one parameter is entered and there is uncompleted raffle.

## /remove_from_blacklist  `bech32_address`
Removes bech32-address from blacklist.
1. `bech32_address` - Cosmos-chain address. <br> Parameter is required.

The command can be executed from any current Discord-server channel. It is safe to execute this command. After that the user does not need to additionally verify this address.

## /raffle_statistic  `raffle_id`
Gives statistics on raffle.
1. `raffle_id` - Id of raffle. <br> The integer number from 1 to 9223372036854775807. <br> Parameter is required.

The command can be executed from any current Discord-server channel. It is safe to execute this command.
