import { appShellEn } from './appShell.en';

const { detailsPage, settings: shellSettings, ...shellRest } = appShellEn;

export default {
  common: {
    emDash: '—',
  },
  gameDetails: {
    patternsCardTitle: 'Patterns',
    patternTagsLabel: 'Pattern tags',
    noPatternTags: 'No pattern tags for this game.',
    systemConnectionLabel: 'System connection',
    noSystemConnection: 'No system-level pattern found yet.',
    ...detailsPage,
  },
  settings: {
    ...shellSettings,
    language: {
      label: 'Language',
      optionEn: 'English',
      optionRu: 'Русский',
    },
  },
  myGames: {
    sync: {
      title: 'Explore your games',
      loadButton: 'Load your games',
    },
    noGames: {
      message: 'There are no games found',
    },
    section: {
      shown: 'Showing {shown} of {total} games',
      noMatching: 'No games match the selected filters',
    },
    toolbar: {
      searchLabel: 'Opponent or opening',
      result: 'Result',
      timeControl: 'Time control',
      period: 'Period',
      pieceColor: 'Your color',
      pattern: 'Pattern',
      opening: 'Opening',
      resetFilters: 'Reset filters',
      results: {
        win: 'Win',
        loss: 'Loss',
        draw: 'Draw',
      },
      speeds: {
        bullet: 'Bullet',
        blitz: 'Blitz',
        rapid: 'Rapid',
        classical: 'Classical',
      },
      periods: {
        d7: '7 days',
        d30: '30 days',
        d90: '90 days',
        all: 'All time',
      },
      colors: {
        white: 'White',
        black: 'Black',
      },
    },
    table: {
      opponent: 'Opponent',
      opening: 'Opening',
      time: 'Time',
      rating: 'Rating',
      accuracy: 'Accuracy',
      acpl: 'ACPL',
      patterns: 'Patterns',
      date: 'Date',
      ariaOpenDetails: 'Open game details',
      ariaOpenPlatform: 'Open on platform',
    },
    speed: {
      bullet: 'Bullet',
      blitz: 'Blitz',
      rapid: 'Rapid',
      classical: 'Classical',
    },
    patterns: {
      lost_winning_position: 'Lost a won game',
      missed_winning_chance: 'Missed a winning chance',
      comeback_win: 'Comeback win',
      opening_blunder: 'Opening blunder',
      middlegame_blunder: 'Middlegame blunder',
      endgame_blunder: 'Endgame blunder',
      multiple_blunders: 'Multiple blunders',
      slow_drift: 'Slow drift (no blunders, high ACPL)',
      low_accuracy: 'Low accuracy',
      late_game_loss: 'Long game loss (≥40 moves)',
      long_loss_min_halfmoves: 'Late loss',
    },
  },
  ...shellRest,
};
