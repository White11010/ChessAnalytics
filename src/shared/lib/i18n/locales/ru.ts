import { appShellRu } from './appShell.ru';

const { detailsPage, settings: shellSettings, ...shellRest } = appShellRu;

export default {
  common: {
    emDash: '—',
  },
  gameDetails: {
    patternsCardTitle: 'Паттерны',
    patternTagsLabel: 'Теги паттернов',
    noPatternTags: 'Для этой партии тегов паттернов нет.',
    systemConnectionLabel: 'Связь с системой',
    noSystemConnection: 'Системный паттерн пока не найден.',
    ...detailsPage,
  },
  settings: {
    ...shellSettings,
    language: {
      label: 'Язык',
      optionEn: 'English',
      optionRu: 'Русский',
    },
  },
  myGames: {
    sync: {
      title: 'Изучайте свои партии',
      loadButton: 'Загрузить партии',
    },
    noGames: {
      message: 'Партии не найдены',
    },
    section: {
      shown: 'Показано {shown} из {total} партий',
      noMatching: 'Нет партий по выбранным фильтрам',
    },
    toolbar: {
      searchLabel: 'Соперник или дебют',
      result: 'Результат',
      timeControl: 'Контроль времени',
      period: 'Период',
      pieceColor: 'Цвет фигур',
      pattern: 'Паттерн',
      opening: 'Дебют',
      resetFilters: 'Сбросить фильтры',
      results: {
        win: 'Победа',
        loss: 'Поражение',
        draw: 'Ничья',
      },
      speeds: {
        bullet: 'Пуля',
        blitz: 'Блиц',
        rapid: 'Рапид',
        classical: 'Классика',
      },
      periods: {
        d7: '7 дней',
        d30: '30 дней',
        d90: '90 дней',
        all: 'Всё время',
      },
      colors: {
        white: 'Белые',
        black: 'Чёрные',
      },
    },
    table: {
      opponent: 'Соперник',
      opening: 'Дебют',
      time: 'Время',
      rating: 'Рейтинг',
      accuracy: 'Точность',
      acpl: 'ACPL',
      patterns: 'Паттерны',
      date: 'Дата',
      ariaOpenDetails: 'Открыть партию',
      ariaOpenPlatform: 'Открыть на платформе',
    },
    speed: {
      bullet: 'Пуля',
      blitz: 'Блиц',
      rapid: 'Рапид',
      classical: 'Классика',
    },
    patterns: {
      lost_winning_position: 'Проиграл выигранную позицию',
      missed_winning_chance: 'Упущенный выигрышный шанс',
      comeback_win: 'Победа после худшей позиции',
      opening_blunder: 'Дебютный зевок',
      middlegame_blunder: 'Зевок в миттельшпиле',
      endgame_blunder: 'Зевок в эндшпиле',
      multiple_blunders: 'Несколько зевков',
      slow_drift: 'Плавное ухудшение (без зевков, высокий ACPL)',
      low_accuracy: 'Низкая точность',
      late_game_loss: 'Поражение в длинной партии (≥40 ходов)',
      long_loss_min_halfmoves: 'Позднее поражение',
    },
  },
  ...shellRest,
};
