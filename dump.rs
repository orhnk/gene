// Permutation

struct Human {
	actions: Vec<Action>
}

enum Action {
	Eat(Food),
	Sleep,
	Code,
}

/*  VS  */

// Combination

struct Human {
	actions: HumanActions
}

struct HumanActions {
	eat: Food,
	sleep: bool,
	code: bool,
}
