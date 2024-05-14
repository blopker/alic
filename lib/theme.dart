import "package:flutter/material.dart";

class MaterialTheme {
  final TextTheme textTheme;

  const MaterialTheme(this.textTheme);

  static MaterialScheme lightScheme() {
    return const MaterialScheme(
      brightness: Brightness.light,
      primary: Color(0xff1b1b1b),
      surfaceTint: Color(0xff5f5e5e),
      onPrimary: Color(0xffffffff),
      primaryContainer: Color(0xff3b3b3b),
      onPrimaryContainer: Color(0xffcfcdcd),
      secondary: Color(0xff5f5e5e),
      onSecondary: Color(0xffffffff),
      secondaryContainer: Color(0xffe9e6e5),
      onSecondaryContainer: Color(0xff4b4a4a),
      tertiary: Color(0xff494949),
      onTertiary: Color(0xffffffff),
      tertiaryContainer: Color(0xff6d6d6d),
      onTertiaryContainer: Color(0xffffffff),
      error: Color(0xffba1a1a),
      onError: Color(0xffffffff),
      errorContainer: Color(0xffffdad6),
      onErrorContainer: Color(0xff410002),
      background: Color(0xfffdf8f8),
      onBackground: Color(0xff1c1b1b),
      surface: Color(0xfffdf8f8),
      onSurface: Color(0xff1c1b1b),
      surfaceVariant: Color(0xffe0e3e3),
      onSurfaceVariant: Color(0xff444748),
      outline: Color(0xff747878),
      outlineVariant: Color(0xffc4c7c7),
      shadow: Color(0xff000000),
      scrim: Color(0xff000000),
      inverseSurface: Color(0xff313030),
      inverseOnSurface: Color(0xfff4f0ef),
      inversePrimary: Color(0xffc8c6c6),
      primaryFixed: Color(0xffe4e2e1),
      onPrimaryFixed: Color(0xff1b1c1c),
      primaryFixedDim: Color(0xffc8c6c6),
      onPrimaryFixedVariant: Color(0xff474747),
      secondaryFixed: Color(0xffe5e2e1),
      onSecondaryFixed: Color(0xff1c1b1b),
      secondaryFixedDim: Color(0xffc9c6c5),
      onSecondaryFixedVariant: Color(0xff474646),
      tertiaryFixed: Color(0xffe4e2e2),
      onTertiaryFixed: Color(0xff1b1c1c),
      tertiaryFixedDim: Color(0xffc7c6c6),
      onTertiaryFixedVariant: Color(0xff464747),
      surfaceDim: Color(0xffddd9d8),
      surfaceBright: Color(0xfffdf8f8),
      surfaceContainerLowest: Color(0xffffffff),
      surfaceContainerLow: Color(0xfff7f3f2),
      surfaceContainer: Color(0xfff1edec),
      surfaceContainerHigh: Color(0xffebe7e7),
      surfaceContainerHighest: Color(0xffe5e2e1),
    );
  }

  ThemeData light() {
    return theme(lightScheme().toColorScheme());
  }

  static MaterialScheme lightMediumContrastScheme() {
    return const MaterialScheme(
      brightness: Brightness.light,
      primary: Color(0xff1b1b1b),
      surfaceTint: Color(0xff5f5e5e),
      onPrimary: Color(0xffffffff),
      primaryContainer: Color(0xff3b3b3b),
      onPrimaryContainer: Color(0xfffcffff),
      secondary: Color(0xff434342),
      onSecondary: Color(0xffffffff),
      secondaryContainer: Color(0xff767474),
      onSecondaryContainer: Color(0xffffffff),
      tertiary: Color(0xff424343),
      onTertiary: Color(0xffffffff),
      tertiaryContainer: Color(0xff6d6d6d),
      onTertiaryContainer: Color(0xffffffff),
      error: Color(0xff8c0009),
      onError: Color(0xffffffff),
      errorContainer: Color(0xffda342e),
      onErrorContainer: Color(0xffffffff),
      background: Color(0xfffdf8f8),
      onBackground: Color(0xff1c1b1b),
      surface: Color(0xfffdf8f8),
      onSurface: Color(0xff1c1b1b),
      surfaceVariant: Color(0xffe0e3e3),
      onSurfaceVariant: Color(0xff404344),
      outline: Color(0xff5c6060),
      outlineVariant: Color(0xff787b7c),
      shadow: Color(0xff000000),
      scrim: Color(0xff000000),
      inverseSurface: Color(0xff313030),
      inverseOnSurface: Color(0xfff4f0ef),
      inversePrimary: Color(0xffc8c6c6),
      primaryFixed: Color(0xff757474),
      onPrimaryFixed: Color(0xffffffff),
      primaryFixedDim: Color(0xff5c5c5c),
      onPrimaryFixedVariant: Color(0xffffffff),
      secondaryFixed: Color(0xff767474),
      onSecondaryFixed: Color(0xffffffff),
      secondaryFixedDim: Color(0xff5d5c5b),
      onSecondaryFixedVariant: Color(0xffffffff),
      tertiaryFixed: Color(0xff757474),
      onTertiaryFixed: Color(0xffffffff),
      tertiaryFixedDim: Color(0xff5c5c5c),
      onTertiaryFixedVariant: Color(0xffffffff),
      surfaceDim: Color(0xffddd9d8),
      surfaceBright: Color(0xfffdf8f8),
      surfaceContainerLowest: Color(0xffffffff),
      surfaceContainerLow: Color(0xfff7f3f2),
      surfaceContainer: Color(0xfff1edec),
      surfaceContainerHigh: Color(0xffebe7e7),
      surfaceContainerHighest: Color(0xffe5e2e1),
    );
  }

  ThemeData lightMediumContrast() {
    return theme(lightMediumContrastScheme().toColorScheme());
  }

  static MaterialScheme lightHighContrastScheme() {
    return const MaterialScheme(
      brightness: Brightness.light,
      primary: Color(0xff1b1b1b),
      surfaceTint: Color(0xff5f5e5e),
      onPrimary: Color(0xffffffff),
      primaryContainer: Color(0xff3b3b3b),
      onPrimaryContainer: Color(0xffffffff),
      secondary: Color(0xff222222),
      onSecondary: Color(0xffffffff),
      secondaryContainer: Color(0xff434342),
      onSecondaryContainer: Color(0xffffffff),
      tertiary: Color(0xff222222),
      onTertiary: Color(0xffffffff),
      tertiaryContainer: Color(0xff424343),
      onTertiaryContainer: Color(0xffffffff),
      error: Color(0xff4e0002),
      onError: Color(0xffffffff),
      errorContainer: Color(0xff8c0009),
      onErrorContainer: Color(0xffffffff),
      background: Color(0xfffdf8f8),
      onBackground: Color(0xff1c1b1b),
      surface: Color(0xfffdf8f8),
      onSurface: Color(0xff000000),
      surfaceVariant: Color(0xffe0e3e3),
      onSurfaceVariant: Color(0xff212525),
      outline: Color(0xff404344),
      outlineVariant: Color(0xff404344),
      shadow: Color(0xff000000),
      scrim: Color(0xff000000),
      inverseSurface: Color(0xff313030),
      inverseOnSurface: Color(0xffffffff),
      inversePrimary: Color(0xffeeebeb),
      primaryFixed: Color(0xff434343),
      onPrimaryFixed: Color(0xffffffff),
      primaryFixedDim: Color(0xff2d2d2d),
      onPrimaryFixedVariant: Color(0xffffffff),
      secondaryFixed: Color(0xff434342),
      onSecondaryFixed: Color(0xffffffff),
      secondaryFixedDim: Color(0xff2d2c2c),
      onSecondaryFixedVariant: Color(0xffffffff),
      tertiaryFixed: Color(0xff424343),
      onTertiaryFixed: Color(0xffffffff),
      tertiaryFixedDim: Color(0xff2c2d2d),
      onTertiaryFixedVariant: Color(0xffffffff),
      surfaceDim: Color(0xffddd9d8),
      surfaceBright: Color(0xfffdf8f8),
      surfaceContainerLowest: Color(0xffffffff),
      surfaceContainerLow: Color(0xfff7f3f2),
      surfaceContainer: Color(0xfff1edec),
      surfaceContainerHigh: Color(0xffebe7e7),
      surfaceContainerHighest: Color(0xffe5e2e1),
    );
  }

  ThemeData lightHighContrast() {
    return theme(lightHighContrastScheme().toColorScheme());
  }

  static MaterialScheme darkScheme() {
    return const MaterialScheme(
      brightness: Brightness.dark,
      primary: Color(0xffc8c6c6),
      surfaceTint: Color(0xffc8c6c6),
      onPrimary: Color(0xff303030),
      primaryContainer: Color(0xff242424),
      onPrimaryContainer: Color(0xffb1afaf),
      secondary: Color(0xffc9c6c5),
      onSecondary: Color(0xff313030),
      secondaryContainer: Color(0xff403f3f),
      onSecondaryContainer: Color(0xffd6d3d3),
      tertiary: Color(0xffc7c6c6),
      onTertiary: Color(0xff303030),
      tertiaryContainer: Color(0xff545454),
      onTertiaryContainer: Color(0xfff9f7f6),
      error: Color(0xffffb4ab),
      onError: Color(0xff690005),
      errorContainer: Color(0xff93000a),
      onErrorContainer: Color(0xffffdad6),
      background: Color(0xff141313),
      onBackground: Color(0xffe5e2e1),
      surface: Color(0xff141313),
      onSurface: Color(0xffe5e2e1),
      surfaceVariant: Color(0xff444748),
      onSurfaceVariant: Color(0xffc4c7c7),
      outline: Color(0xff8e9192),
      outlineVariant: Color(0xff444748),
      shadow: Color(0xff000000),
      scrim: Color(0xff000000),
      inverseSurface: Color(0xffe5e2e1),
      inverseOnSurface: Color(0xff313030),
      inversePrimary: Color(0xff5f5e5e),
      primaryFixed: Color(0xffe4e2e1),
      onPrimaryFixed: Color(0xff1b1c1c),
      primaryFixedDim: Color(0xffc8c6c6),
      onPrimaryFixedVariant: Color(0xff474747),
      secondaryFixed: Color(0xffe5e2e1),
      onSecondaryFixed: Color(0xff1c1b1b),
      secondaryFixedDim: Color(0xffc9c6c5),
      onSecondaryFixedVariant: Color(0xff474646),
      tertiaryFixed: Color(0xffe4e2e2),
      onTertiaryFixed: Color(0xff1b1c1c),
      tertiaryFixedDim: Color(0xffc7c6c6),
      onTertiaryFixedVariant: Color(0xff464747),
      surfaceDim: Color(0xff141313),
      surfaceBright: Color(0xff3a3939),
      surfaceContainerLowest: Color(0xff0e0e0e),
      surfaceContainerLow: Color(0xff1c1b1b),
      surfaceContainer: Color(0xff201f1f),
      surfaceContainerHigh: Color(0xff2b2a2a),
      surfaceContainerHighest: Color(0xff353434),
    );
  }

  ThemeData dark() {
    return theme(darkScheme().toColorScheme());
  }

  static MaterialScheme darkMediumContrastScheme() {
    return const MaterialScheme(
      brightness: Brightness.dark,
      primary: Color(0xffcccaca),
      surfaceTint: Color(0xffc8c6c6),
      onPrimary: Color(0xff161616),
      primaryContainer: Color(0xff929090),
      onPrimaryContainer: Color(0xff000000),
      secondary: Color(0xffcdcac9),
      onSecondary: Color(0xff161616),
      secondaryContainer: Color(0xff929090),
      onSecondaryContainer: Color(0xff000000),
      tertiary: Color(0xffcccaca),
      onTertiary: Color(0xff161617),
      tertiaryContainer: Color(0xff919090),
      onTertiaryContainer: Color(0xff000000),
      error: Color(0xffffbab1),
      onError: Color(0xff370001),
      errorContainer: Color(0xffff5449),
      onErrorContainer: Color(0xff000000),
      background: Color(0xff141313),
      onBackground: Color(0xffe5e2e1),
      surface: Color(0xff141313),
      onSurface: Color(0xfffefaf9),
      surfaceVariant: Color(0xff444748),
      onSurfaceVariant: Color(0xffc8cbcc),
      outline: Color(0xffa0a3a4),
      outlineVariant: Color(0xff808484),
      shadow: Color(0xff000000),
      scrim: Color(0xff000000),
      inverseSurface: Color(0xffe5e2e1),
      inverseOnSurface: Color(0xff2b2a2a),
      inversePrimary: Color(0xff484848),
      primaryFixed: Color(0xffe4e2e1),
      onPrimaryFixed: Color(0xff111111),
      primaryFixedDim: Color(0xffc8c6c6),
      onPrimaryFixedVariant: Color(0xff363636),
      secondaryFixed: Color(0xffe5e2e1),
      onSecondaryFixed: Color(0xff111111),
      secondaryFixedDim: Color(0xffc9c6c5),
      onSecondaryFixedVariant: Color(0xff373636),
      tertiaryFixed: Color(0xffe4e2e2),
      onTertiaryFixed: Color(0xff101111),
      tertiaryFixedDim: Color(0xffc7c6c6),
      onTertiaryFixedVariant: Color(0xff363636),
      surfaceDim: Color(0xff141313),
      surfaceBright: Color(0xff3a3939),
      surfaceContainerLowest: Color(0xff0e0e0e),
      surfaceContainerLow: Color(0xff1c1b1b),
      surfaceContainer: Color(0xff201f1f),
      surfaceContainerHigh: Color(0xff2b2a2a),
      surfaceContainerHighest: Color(0xff353434),
    );
  }

  ThemeData darkMediumContrast() {
    return theme(darkMediumContrastScheme().toColorScheme());
  }

  static MaterialScheme darkHighContrastScheme() {
    return const MaterialScheme(
      brightness: Brightness.dark,
      primary: Color(0xfffdfaf9),
      surfaceTint: Color(0xffc8c6c6),
      onPrimary: Color(0xff000000),
      primaryContainer: Color(0xffcccaca),
      onPrimaryContainer: Color(0xff000000),
      secondary: Color(0xfffefaf9),
      onSecondary: Color(0xff000000),
      secondaryContainer: Color(0xffcdcac9),
      onSecondaryContainer: Color(0xff000000),
      tertiary: Color(0xfffcfafa),
      onTertiary: Color(0xff000000),
      tertiaryContainer: Color(0xffcccaca),
      onTertiaryContainer: Color(0xff000000),
      error: Color(0xfffff9f9),
      onError: Color(0xff000000),
      errorContainer: Color(0xffffbab1),
      onErrorContainer: Color(0xff000000),
      background: Color(0xff141313),
      onBackground: Color(0xffe5e2e1),
      surface: Color(0xff141313),
      onSurface: Color(0xffffffff),
      surfaceVariant: Color(0xff444748),
      onSurfaceVariant: Color(0xfff9fbfb),
      outline: Color(0xffc8cbcc),
      outlineVariant: Color(0xffc8cbcc),
      shadow: Color(0xff000000),
      scrim: Color(0xff000000),
      inverseSurface: Color(0xffe5e2e1),
      inverseOnSurface: Color(0xff000000),
      inversePrimary: Color(0xff2a2a2a),
      primaryFixed: Color(0xffe9e6e6),
      onPrimaryFixed: Color(0xff000000),
      primaryFixedDim: Color(0xffcccaca),
      onPrimaryFixedVariant: Color(0xff161616),
      secondaryFixed: Color(0xffe9e6e5),
      onSecondaryFixed: Color(0xff000000),
      secondaryFixedDim: Color(0xffcdcac9),
      onSecondaryFixedVariant: Color(0xff161616),
      tertiaryFixed: Color(0xffe8e6e6),
      onTertiaryFixed: Color(0xff000000),
      tertiaryFixedDim: Color(0xffcccaca),
      onTertiaryFixedVariant: Color(0xff161617),
      surfaceDim: Color(0xff141313),
      surfaceBright: Color(0xff3a3939),
      surfaceContainerLowest: Color(0xff0e0e0e),
      surfaceContainerLow: Color(0xff1c1b1b),
      surfaceContainer: Color(0xff201f1f),
      surfaceContainerHigh: Color(0xff2b2a2a),
      surfaceContainerHighest: Color(0xff353434),
    );
  }

  ThemeData darkHighContrast() {
    return theme(darkHighContrastScheme().toColorScheme());
  }

  ThemeData theme(ColorScheme colorScheme) => ThemeData(
        useMaterial3: true,
        brightness: colorScheme.brightness,
        colorScheme: colorScheme,
        textTheme: textTheme.apply(
          bodyColor: colorScheme.onSurface,
          displayColor: colorScheme.onSurface,
        ),
        scaffoldBackgroundColor: colorScheme.surface,
        canvasColor: colorScheme.surface,
      );

  List<ExtendedColor> get extendedColors => [];
}

class MaterialScheme {
  const MaterialScheme({
    required this.brightness,
    required this.primary,
    required this.surfaceTint,
    required this.onPrimary,
    required this.primaryContainer,
    required this.onPrimaryContainer,
    required this.secondary,
    required this.onSecondary,
    required this.secondaryContainer,
    required this.onSecondaryContainer,
    required this.tertiary,
    required this.onTertiary,
    required this.tertiaryContainer,
    required this.onTertiaryContainer,
    required this.error,
    required this.onError,
    required this.errorContainer,
    required this.onErrorContainer,
    required this.background,
    required this.onBackground,
    required this.surface,
    required this.onSurface,
    required this.surfaceVariant,
    required this.onSurfaceVariant,
    required this.outline,
    required this.outlineVariant,
    required this.shadow,
    required this.scrim,
    required this.inverseSurface,
    required this.inverseOnSurface,
    required this.inversePrimary,
    required this.primaryFixed,
    required this.onPrimaryFixed,
    required this.primaryFixedDim,
    required this.onPrimaryFixedVariant,
    required this.secondaryFixed,
    required this.onSecondaryFixed,
    required this.secondaryFixedDim,
    required this.onSecondaryFixedVariant,
    required this.tertiaryFixed,
    required this.onTertiaryFixed,
    required this.tertiaryFixedDim,
    required this.onTertiaryFixedVariant,
    required this.surfaceDim,
    required this.surfaceBright,
    required this.surfaceContainerLowest,
    required this.surfaceContainerLow,
    required this.surfaceContainer,
    required this.surfaceContainerHigh,
    required this.surfaceContainerHighest,
  });

  final Brightness brightness;
  final Color primary;
  final Color surfaceTint;
  final Color onPrimary;
  final Color primaryContainer;
  final Color onPrimaryContainer;
  final Color secondary;
  final Color onSecondary;
  final Color secondaryContainer;
  final Color onSecondaryContainer;
  final Color tertiary;
  final Color onTertiary;
  final Color tertiaryContainer;
  final Color onTertiaryContainer;
  final Color error;
  final Color onError;
  final Color errorContainer;
  final Color onErrorContainer;
  final Color background;
  final Color onBackground;
  final Color surface;
  final Color onSurface;
  final Color surfaceVariant;
  final Color onSurfaceVariant;
  final Color outline;
  final Color outlineVariant;
  final Color shadow;
  final Color scrim;
  final Color inverseSurface;
  final Color inverseOnSurface;
  final Color inversePrimary;
  final Color primaryFixed;
  final Color onPrimaryFixed;
  final Color primaryFixedDim;
  final Color onPrimaryFixedVariant;
  final Color secondaryFixed;
  final Color onSecondaryFixed;
  final Color secondaryFixedDim;
  final Color onSecondaryFixedVariant;
  final Color tertiaryFixed;
  final Color onTertiaryFixed;
  final Color tertiaryFixedDim;
  final Color onTertiaryFixedVariant;
  final Color surfaceDim;
  final Color surfaceBright;
  final Color surfaceContainerLowest;
  final Color surfaceContainerLow;
  final Color surfaceContainer;
  final Color surfaceContainerHigh;
  final Color surfaceContainerHighest;
}

extension MaterialSchemeUtils on MaterialScheme {
  ColorScheme toColorScheme() {
    return ColorScheme(
      brightness: brightness,
      primary: primary,
      onPrimary: onPrimary,
      primaryContainer: primaryContainer,
      onPrimaryContainer: onPrimaryContainer,
      secondary: secondary,
      onSecondary: onSecondary,
      secondaryContainer: secondaryContainer,
      onSecondaryContainer: onSecondaryContainer,
      tertiary: tertiary,
      onTertiary: onTertiary,
      tertiaryContainer: tertiaryContainer,
      onTertiaryContainer: onTertiaryContainer,
      error: error,
      onError: onError,
      errorContainer: errorContainer,
      onErrorContainer: onErrorContainer,
      surface: surface,
      onSurface: onSurface,
      surfaceContainerHighest: surfaceVariant,
      onSurfaceVariant: onSurfaceVariant,
      outline: outline,
      outlineVariant: outlineVariant,
      shadow: shadow,
      scrim: scrim,
      inverseSurface: inverseSurface,
      onInverseSurface: inverseOnSurface,
      inversePrimary: inversePrimary,
    );
  }
}

class ExtendedColor {
  final Color seed, value;
  final ColorFamily light;
  final ColorFamily lightHighContrast;
  final ColorFamily lightMediumContrast;
  final ColorFamily dark;
  final ColorFamily darkHighContrast;
  final ColorFamily darkMediumContrast;

  const ExtendedColor({
    required this.seed,
    required this.value,
    required this.light,
    required this.lightHighContrast,
    required this.lightMediumContrast,
    required this.dark,
    required this.darkHighContrast,
    required this.darkMediumContrast,
  });
}

class ColorFamily {
  const ColorFamily({
    required this.color,
    required this.onColor,
    required this.colorContainer,
    required this.onColorContainer,
  });

  final Color color;
  final Color onColor;
  final Color colorContainer;
  final Color onColorContainer;
}
