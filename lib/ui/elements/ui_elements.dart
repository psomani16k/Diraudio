import 'package:flutter/material.dart';

class DiraudioUiElements {
  static Widget diraudiTonalButton(
      BuildContext context, String text, double width, Function onTap) {
    return InkWell(
      onTap: () {
        onTap();
      },
      borderRadius: BorderRadius.circular(20),
      hoverColor: Theme.of(context).colorScheme.secondaryContainer,
      splashColor:
          Theme.of(context).colorScheme.secondaryContainer.withAlpha(100),
      child: Container(
        height: 48,
        width: width,
        decoration: BoxDecoration(
            // color: Theme.of(context).colorScheme.secondaryContainer,
            border: Border.all(
                color: Theme.of(context).colorScheme.secondaryContainer,
                width: 2),
            borderRadius: BorderRadius.circular(20)),
        child: Center(child: Text(text)),
      ),
    );
  }

  static Widget diraudioFIlledButton(
      BuildContext context, Widget child, double width, Function onTap) {
    return InkWell(
      onTap: () {
        onTap();
      },
      borderRadius: BorderRadius.circular(20),
      child: Container(
        height: 48,
        width: width,
        decoration: BoxDecoration(
            boxShadow: const [
              BoxShadow(
                  color: Colors.black38, blurRadius: 10, offset: Offset(2, 2))
            ],
            color: Theme.of(context).colorScheme.primaryContainer,
            borderRadius: BorderRadius.circular(20)),
        child: Center(child: child),
      ),
    );
  }

  /// 0 < progress < 1
  /// otherwise a modulo will be taken
  static Widget diraudioProgressIndicator(
      BuildContext context, double progress, double width) {
    width = width - 7;
    if (progress > 1) {
      progress = progress % 1;
    }
    return Row(
      mainAxisAlignment: MainAxisAlignment.start,
      children: [
        Container(
          height: 4,
          width: width * progress,
          decoration: BoxDecoration(
            color: Theme.of(context).colorScheme.primaryContainer,
            borderRadius: BorderRadius.circular(4),
          ),
        ),
        Padding(
          padding: const EdgeInsets.symmetric(horizontal: 2),
          child: Container(
            width: 3,
            height: 12,
            decoration: BoxDecoration(
              borderRadius: BorderRadius.circular(2),
              color: Theme.of(context).colorScheme.primaryContainer,
            ),
          ),
        ),
        Container(
          height: 4,
          width: width * (1 - progress),
          decoration: BoxDecoration(
            color: Theme.of(context).colorScheme.secondaryContainer,
            borderRadius: BorderRadius.circular(4),
          ),
        )
      ],
    );
  }
}
