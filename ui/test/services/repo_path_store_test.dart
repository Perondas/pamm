import 'package:flutter_test/flutter_test.dart';
import 'package:get_it/get_it.dart';
import 'package:pamm_ui/src/services/repo_path_store.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:shared_preferences_platform_interface/in_memory_shared_preferences_async.dart';
import 'package:shared_preferences_platform_interface/shared_preferences_async_platform_interface.dart';

void main() {
  setUp(() async {
    SharedPreferencesAsyncPlatform.instance =
        InMemorySharedPreferencesAsync.empty();
    await GetIt.instance.reset();
    GetIt.instance.registerSingletonAsync<SharedPreferencesWithCache>(() {
      return SharedPreferencesWithCache.create(
        cacheOptions: const SharedPreferencesWithCacheOptions(),
      );
    });
    await GetIt.instance.allReady();
  });

  test('starts out empty', () async {
    expect(await RepoPathStore.getRepoPaths(), isEmpty);
  });

  test('added paths are returned', () async {
    await RepoPathStore.add('/repos/one');
    await RepoPathStore.add('/repos/two');

    expect(
      await RepoPathStore.getRepoPaths(),
      {'/repos/one', '/repos/two'},
    );
  });

  test('adding the same path twice stores it once', () async {
    await RepoPathStore.add('/repos/one');
    await RepoPathStore.add('/repos/one');

    expect(await RepoPathStore.getRepoPaths(), {'/repos/one'});
  });

  test('removed paths are no longer returned', () async {
    await RepoPathStore.add('/repos/one');
    await RepoPathStore.add('/repos/two');

    await RepoPathStore.remove('/repos/one');

    expect(await RepoPathStore.getRepoPaths(), {'/repos/two'});
  });

  test('removing an unknown path leaves the store unchanged', () async {
    await RepoPathStore.add('/repos/one');

    await RepoPathStore.remove('/repos/does-not-exist');

    expect(await RepoPathStore.getRepoPaths(), {'/repos/one'});
  });

  test('clear removes all paths', () async {
    await RepoPathStore.add('/repos/one');
    await RepoPathStore.add('/repos/two');

    await RepoPathStore.clear();

    expect(await RepoPathStore.getRepoPaths(), isEmpty);
  });
}
