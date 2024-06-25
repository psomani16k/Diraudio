import 'package:flutter/material.dart';

class DiraudioUiElements {
  static diraudiTonalButton(
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

  static diraudioFIlledButton(
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
}
